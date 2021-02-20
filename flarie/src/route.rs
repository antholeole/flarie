use crate::response::Response;

pub struct Route<T> {
    executor: fn() -> Response,
    path: &'static str,
    path_params: Option<T>,
    body: Option<String>,
}

impl<T> Route<T> {
    pub fn new(executor: fn() -> Response, path: &'static str) -> Route<T> {
        Route {
            executor,
            path,
            body: None,
            path_params: None
        }
    }

    pub fn matcher(&self, match_to: &str) -> bool {
        self.path == match_to
    }

    pub fn enrich(mut self, body: Option<String>, path_params: T) {
        self.body = body;
        self.path_params = Some(path_params);
    }

    pub fn run(&self) -> Response {
        (self.executor)()
    }
}
