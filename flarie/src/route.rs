use crate::errors::FlarieError;
use crate::response::Response;
use itertools::{EitherOrBoth::Both, Itertools};

enum Matcher<'a> {
    PathParam(&'a str),
    Constant,
}

enum RouteType {
    Get,
    Post,
    Delete,
    Put,
    Patch,
    None
}

pub struct RouteData<T> {
    pub path_params: Option<T>,
    get_path_params: fn(Vec<&str>) -> Option<T>,
    pub body: Option<String>,
    route_type: RouteType
}

impl<T> RouteData<T> {
    pub fn empty(str_to_path_params: fn(Vec<&str>) -> Option<T>) -> RouteData<T> {
        RouteData {
            path_params: None,
            body: None,
            get_path_params: str_to_path_params,
            route_type: RouteType::None,
        }
    }
}

pub struct Route<T> {
    executor: fn(RouteData<T>) -> Response,
    path: &'static str,
    route_data: RouteData<T>,
}

impl<T> Route<T> {
    pub fn new(
        executor: fn(RouteData<T>) -> Response,
        match_path_params: fn(Vec<&str>) -> Option<T>,
        path: &'static str,
    ) -> Route<T> {
        Route {
            executor,
            path,
            route_data: RouteData::empty(match_path_params),
        }
    }

    //returns Some(path_params) if matches, None if doesn't
    pub fn matcher(&self, match_to: &str) -> Option<T> {
        //will short circuit if a path does not match
        let t = match_to
            .split_terminator('/')
            .zip_longest(self.path.split('/'))
            .map(|pair| match pair {
                Both(input_path, should_be_path) => {
                    if should_be_path.starts_with('{') && should_be_path.ends_with('}') {
                        Ok(Matcher::PathParam(input_path))
                    } else if should_be_path == input_path {
                        Ok(Matcher::Constant)
                    } else {
                        Err(FlarieError::PathsDontMatchError)
                    }
                }
                _ => Err(FlarieError::PathsDontMatchError),
            })
            .filter(|v| {
                //filter out all non-path params, as we are done w/ them
                match v {
                    Ok(matcher) => matches!(matcher, Matcher::PathParam(_)),
                    _ => true, //dont want to filter out our error varient or else we wont short circuit
                }
            })
            .map(|v| {
                //turn all path params into inner
                match v {
                    Ok(matcher) => match matcher {
                        Matcher::PathParam(p) => Ok(p),
                        _ => panic!("impossible to get here"),
                    },
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<Vec<&str>, FlarieError>>();

        match t {
            Ok(v) => (self.route_data.get_path_params)(v),
            Err(_) => None,
        }
    }

    pub fn enrich(mut self, body: Option<String>, path_params: T) -> Self {
        self.route_data.body = body;
        self.route_data.path_params = Some(path_params);
        self
    }

    pub fn run(self) -> Response {
        (self.executor)(self.route_data)
    }
}
