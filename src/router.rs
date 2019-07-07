pub enum Tree<T> {
    Wildcard(Vec<Tree<T>>),
    Specific(String, Vec<Tree<T>>),
    Parameter(String, Vec<Tree<T>>),
    Leaf(T),
}

pub struct Route<'r> {
    pub Path: Vec<&'r str>,
}

pub struct Router<T> {
    pub tree: Tree<T>,
}

fn find_matching_child<T>(
    children: &mut Vec<Tree<T>>,
    route: &Route,
    level: usize,
) -> Option<usize> {
    let mut child_to_add_to: Option<usize> = None;

    let mut counter = 0;

    let mut idx = 0;
    for c in children {
        match c {
            Tree::Leaf(_) => {
                //TODO error types
                panic!("Tried to add as a neighbor to a leaf");
            }
            Tree::Wildcard(_) => {
                if route.Path[level] != "*" {
                    //TODO error types
                    panic!("Tried to add are more specific route as a neighbor to a wildcard");
                } else {
                    child_to_add_to = Some(idx);
                    counter += 1;
                }
            }
            Tree::Specific(name, _) => {
                if route.Path[level] == name.as_str() {
                    child_to_add_to = Some(idx);
                    counter += 1;
                }
            }
            Tree::Parameter(name, _) => {
                if route.Path[level] == name.as_str() {
                    child_to_add_to = Some(idx);
                    counter += 1;
                } else {
                    //TODO error types
                    panic!("Tried to add something other than the same variable on the path");
                }
            }
        }
        idx += 1;
    }

    if counter > 1 {
        panic!("Bug detected");
    }

    child_to_add_to
}

fn add_route<T>(tree: &mut Tree<T>, route: &Route, level: usize, item: T) {
    let children = match tree {
        Tree::Leaf(_) => {
            //TODO error types
            panic!("Tried to add to leaf");
        }
        Tree::Specific(_, children) => (children),
        Tree::Parameter(_, children) => (children),
        Tree::Wildcard(children) => (children),
    };

    if level == route.Path.len() {
        if children.len() > 0 {
            panic!("Tried to add leave on path that already has lower leaves");
        }
        children.push(Tree::Leaf(item));
    } else {
        println!("{}", route.Path[level]);

        let idx = find_matching_child(children, route, level);

        match idx {
            Some(idx) => {
                add_route(&mut children[idx], route, level + 1, item);
            }
            None => {
                //need to add new child depending on the part of the route
                let name: &str = route.Path[level];
                if name == "*" {
                    if children.len() > 0 {
                        panic!("Tried to add wildcard on path that already has lower leaves");
                    }
                    children.push(Tree::Wildcard(Vec::new()));
                } else {
                    if name.starts_with(":") {
                        if children.len() > 0 {
                            panic!("Tried to add parameter on path that already has lower leaves");
                        }
                        children.push(Tree::Parameter(name.to_owned(), Vec::new()));
                    } else {
                        children.push(Tree::Specific(name.to_owned(), Vec::new()));
                    }
                }
                add_route(children.last_mut().unwrap(), route, level + 1, item);
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
    if level == route.Path.len() {
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
                if name.as_str() == route.Path[level] {
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
                            params.insert(name.to_owned(), route.Path[level].to_owned());
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
    pub fn add_route(&mut self, route: &Route, item: T) {
        add_route(&mut self.tree, route, 0, item);
    }

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
