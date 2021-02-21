use crate::response::Response;
use itertools::{Itertools, EitherOrBoth::Both};
use crate::errors::FlarieError;

enum Matcher<'a> {
    PathParam(&'a str),
    Constant,
}

pub struct RouteData<T> {
    path_params: Option<T>,
    get_path_params: fn(Vec<&str>) -> Option<T>,
    body: Option<String>,
}

impl <T> RouteData<T> {
    pub fn empty(str_to_path_params: fn(Vec<&str>) -> Option<T>) -> RouteData<T> {
        RouteData {
            path_params: None,
            body: None,
            get_path_params: str_to_path_params
        }
    }

    pub fn with_path_params(mut self, path_params: Option<T>) -> Self {
        self.path_params = path_params;
        self
    }

    pub fn with_body(mut self, body: Option<String>) -> Self {
        self.body = body;
        self
    }
}

pub struct Route<T, F> where F: Fn(RouteData<T>) -> Response {
    executor: F,
    path: &'static str,
    route_data: RouteData<T>
}

impl<T, F> Route<T, F> where F: Fn(RouteData<T>) -> Response {
    pub fn new(executor: F, match_path_params: fn(Vec<&str>) -> Option<T>, path: &'static str) -> Route<T, F> {
        Route {
            executor,
            path,
            route_data: RouteData::empty(match_path_params),
        }
    }

    //returns Some(path_params) if matches, None if doesn't
    pub fn matcher(&self, match_to: &str) -> Option<T> {
        let t = match_to.split_terminator("/").zip_longest(self.path.split("/")).map(|pair| {
            match pair {
                Both(input_path, should_be_path) => {
                    if should_be_path.starts_with("{") && should_be_path.ends_with("}") {
                        Ok(Matcher::PathParam(input_path))
                    } else if should_be_path == input_path {
                        Ok(Matcher::Constant)
                    } else {
                        Err(FlarieError::PathsDontMatchError)
                    }
                },
                _ => Err(FlarieError::PathsDontMatchError)
            }
        }).filter(|v| { //filter out all non-path params, as we are done w/ them
            match v {
                Ok(matcher) => match matcher {
                    Matcher::PathParam(_) => true,
                    _ => false
                },
                _ => true //dont want to filter out our error varient or else we wont short circuit
            }
        }).map(|v| { //turn all path params into inner
            match v {
                Ok(matcher) => match matcher {
                    Matcher::PathParam(p) => Ok(p),
                    _ => panic!("impossible to get here"),
                }
                Err(e) => Err(e)
            }
        }).collect::<Result::<Vec::<&str>, FlarieError>>();

        match t {
            Ok(v) => (self.route_data.get_path_params)(v),
            Err(_) => None
        }
    }

    pub fn enrich(mut self, body: Option<String>) {
        self.route_data.body = body;
    }

    pub fn run(mut self) -> Response {
        //TODO cleanup unwrap
        (self.executor)(self.route_data)
    }
}
