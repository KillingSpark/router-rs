use std::fmt;

pub struct Route<'r> {
    pub path: Vec<&'r str>,
}

#[derive(Debug, Clone)]
pub struct MalformedRouteError(String);

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
