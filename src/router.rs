use super::route::Route;
use std::fmt;

pub enum Tree<T> {
    Wildcard(Vec<Tree<T>>),
    Specific(String, Vec<Tree<T>>),
    Parameter(String, Vec<Tree<T>>),
    Leaf(T, bool),
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
pub enum AddRouteError {
    AddToLeafLevel,
    MismatchTypes(String, String),
    MismatchParameter(String, String),
}

impl fmt::Display for AddRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddRouteError::AddToLeafLevel => write!(
                f,
                "tried to add a path so that a leaf would be on another longer path"
            ),
            AddRouteError::MismatchTypes(t1, t2) => write!(
                f,
                "tried to add path so that two different types of parts collide: {} and {}",
                t1, t2
            ),
            AddRouteError::MismatchParameter(t1, t2) => write!(
                f,
                "tried to add path so that a paramter and another part collide: {} and {}",
                t1, t2
            ),
        }
    }
}

impl std::error::Error for AddRouteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn find_matching_child<T>(
    children: &mut Vec<Tree<T>>,
    route: &Route,
    level: usize,
) -> Result<Option<usize>, AddRouteError> {
    let mut child_to_add_to: Option<usize> = None;

    let mut counter = 0;

    let mut idx = 0;
    for c in children {
        match c {
            Tree::Leaf(_, _) => {
                return Err(AddRouteError::AddToLeafLevel);
            }
            Tree::Wildcard(_) => {
                if route.path[level] != "*" {
                    return Err(AddRouteError::MismatchTypes(
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
                    return Err(AddRouteError::MismatchParameter(
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
) -> Result<(), AddRouteError> {
    let children = match tree {
        Tree::Leaf(_, _) => {
            return Err(AddRouteError::AddToLeafLevel);
        }
        Tree::Specific(_, children) => (children),
        Tree::Parameter(_, children) => (children),
        Tree::Wildcard(children) => (children),
    };

    if level == route.path.len() {
        if children.len() > 0 {
            return Err(AddRouteError::AddToLeafLevel);
        }
        let chatch_all = route.path[level - 1] == "*";
        children.push(Tree::Leaf(item, chatch_all));
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
                                return Err(AddRouteError::MismatchTypes(
                                    "Specific/Parameter".to_owned(),
                                    "Wildcard".to_owned(),
                                ));
                            }
                            children.push(Tree::Wildcard(Vec::new()));
                        } else {
                            if name.starts_with(":") {
                                if children.len() > 0 {
                                    return Err(AddRouteError::MismatchParameter(
                                        name.to_owned(),
                                        "other".to_owned(),
                                    ));
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
            Tree::Leaf(item, _) => return Some(item),
            _ => return None, //this path is longer than the wanted route
        }
    } else {
        match tree {
            Tree::Leaf(item, catch_all) => return if *catch_all { Some(item) } else { None }, //this path is shorter than the wanted route
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
    pub fn add_route(&mut self, route: &Route, item: T) -> Result<(), AddRouteError> {
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
            //buggy -> panic
            _ => panic!("Corrupt root"),
        };

        None
    }
}
