pub enum Tree<T> {
    Wildcard(Vec<Tree<T>>),
    Specific(String, Vec<Tree<T>>),
    Parameter(String, Vec<Tree<T>>),
    Leaf(T),
}

pub struct Route<'r> {
    path: Vec<&'r str>,
}

#[allow(dead_code)]
pub fn new_route(p: &str) -> Result<Route, MalformedRouteError> {
    //TODO error type
    if p.len() < 1 {
        return Err(MalformedRouteError(p.to_owned()));
    }
    if !p.starts_with("/") {
        return Err(MalformedRouteError(p.to_owned()));
    }
    if p.ends_with("/") {
        //maybe just trim? Not sure
        return Err(MalformedRouteError(p.to_owned()));
    }
    Ok(Route {
        path: p.split("/").collect(),
    })
}

pub struct Router<T> {
    pub tree: Tree<T>,
}

#[allow(dead_code)]
pub fn new_router<T>() -> Router<T> {
    Router {
        tree: Tree::Wildcard(Vec::new()),
    }
}

#[derive(Debug, Clone)]
pub enum RouteError {
    AddToLeafLevel,
    MismatchTypes(String, String),
    MismatchParameter(String, String),
}

#[derive(Debug, Clone)]
pub struct MalformedRouteError(String);

use std::fmt;
impl fmt::Display for MalformedRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This route is malformed: {}", self.0)
    }
}

impl std::error::Error for MalformedRouteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RouteError::AddToLeafLevel => write!(
                f,
                "tried to add a path so that a leaf would be on another longer path"
            ),
            RouteError::MismatchTypes(t1, t2) => write!(
                f,
                "tried to add path so that two different types of parts collide: {} and {}",
                t1, t2
            ),
            RouteError::MismatchParameter(t1, t2) => write!(
                f,
                "tried to add path so that a paramter and another part collide: {} and {}",
                t1, t2
            ),
        }
    }
}

impl std::error::Error for RouteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn find_matching_child<T>(
    children: &mut Vec<Tree<T>>,
    route: &Route,
    level: usize,
) -> Result<Option<usize>, RouteError> {
    let mut child_to_add_to: Option<usize> = None;

    let mut counter = 0;

    let mut idx = 0;
    for c in children {
        match c {
            Tree::Leaf(_) => {
                return Err(RouteError::AddToLeafLevel);
            }
            Tree::Wildcard(_) => {
                if route.path[level] != "*" {
                    return Err(RouteError::MismatchTypes(
                        "Wildcard".to_owned(),
                        "Specific".to_owned(),
                    ));
                } else {
                    child_to_add_to = Some(idx);
                    counter += 1;
                }
            }
            Tree::Specific(name, _) => {
                if route.path[level] == name.as_str() {
                    child_to_add_to = Some(idx);
                    counter += 1;
                }
            }
            Tree::Parameter(name, _) => {
                if route.path[level] == name.as_str() {
                    child_to_add_to = Some(idx);
                    counter += 1;
                } else {
                    return Err(RouteError::MismatchParameter(
                        name.clone(),
                        route.path[level].to_owned(),
                    ));
                }
            }
        }
        idx += 1;
    }

    if counter > 1 {
        panic!("Bug detected");
    }

    Ok(child_to_add_to)
}

fn add_route<T>(
    tree: &mut Tree<T>,
    route: &Route,
    level: usize,
    item: T,
) -> Result<(), RouteError> {
    let children = match tree {
        Tree::Leaf(_) => {
            return Err(RouteError::AddToLeafLevel);
        }
        Tree::Specific(_, children) => (children),
        Tree::Parameter(_, children) => (children),
        Tree::Wildcard(children) => (children),
    };

    if level == route.path.len() {
        if children.len() > 0 {
            return Err(RouteError::AddToLeafLevel);
        }
        children.push(Tree::Leaf(item));
        Ok(())
    } else {
        match find_matching_child(children, route, level) {
            Err(e) => return Err(e),
            Ok(idx) => {
                match idx {
                    Some(idx) => {
                        return add_route(&mut children[idx], route, level + 1, item);
                    }
                    None => {
                        //need to add new child depending on the part of the route
                        let name: &str = route.path[level];
                        if name == "*" {
                            if children.len() > 0 {
                                return Err(RouteError::MismatchTypes("Specific/Parameter".to_owned(), "Wildcard".to_owned()));
                            }
                            children.push(Tree::Wildcard(Vec::new()));
                        } else {
                            if name.starts_with(":") {
                                if children.len() > 0 {
                                    return Err(RouteError::MismatchParameter(name.to_owned(), "other".to_owned()));
                                }
                                children.push(Tree::Parameter(name.to_owned(), Vec::new()));
                            } else {
                                children.push(Tree::Specific(name.to_owned(), Vec::new()));
                            }
                        }
                        return add_route(children.last_mut().unwrap(), route, level + 1, item);
                    }
                }
            }
        }
    }
}

use std::collections::HashMap;
fn find_route<'a, T>(
    tree: &'a mut Tree<T>,
    route: &Route,
    level: usize,
    params: &mut HashMap<String, String>,
) -> Option<&'a mut T> {
    if level == route.path.len() {
        match tree {
            Tree::Leaf(item) => return Some(item),
            _ => panic!("No leaf on this route's end"),
        }
    } else {
        match tree {
            Tree::Leaf(_) => {
                //TODO error types
                panic!("Tried to get children of a leaf");
            }
            Tree::Specific(name, children) => {
                if name.as_str() == route.path[level] {
                    for c in children {
                        match find_route(c, route, level + 1, params) {
                            Some(r) => return Some(r),
                            None => {}
                        }
                    }
                }
            }
            Tree::Parameter(name, children) => {
                for c in children {
                    match find_route(c, route, level + 1, params) {
                        Some(r) => {
                            params.insert(name.to_owned(), route.path[level].to_owned());
                            return Some(r);
                        }
                        None => {}
                    }
                }
            }

            Tree::Wildcard(children) => {
                for c in children {
                    match find_route(c, route, level + 1, params) {
                        Some(r) => return Some(r),
                        None => {}
                    }
                }
            }
        };
        None
    }
}

impl<T> Router<T> {
    #[allow(dead_code)]
    pub fn add_route(&mut self, route: &Route, item: T) -> Result<(), RouteError>{
        add_route(&mut self.tree, route, 0, item)
    }

    #[allow(dead_code)]
    pub fn route<'a>(&'a mut self, route: &Route) -> Option<(&'a mut T, HashMap<String, String>)> {
        match &mut self.tree {
            Tree::Wildcard(root_children) => {
                let mut params = HashMap::new();
                for c in root_children {
                    params.clear();
                    let item = find_route(c, route, 0, &mut params);
                    match item {
                        Some(i) => return Some((i, params)),
                        None => {}
                    }
                }
            }
            _ => panic!("Corrupt root"),
        };

        None
    }
}
