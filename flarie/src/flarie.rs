use crate::route::{Route, RouteData};
use crate::response::Response;
use crate::errors::FlarieError;

struct Request {
    path: String,
}

pub struct Flarie {
    found: bool,
    request: Request,
}

pub type Path<T> = T;

impl Flarie {
    pub fn new(obj: &str) -> Self {
        Flarie {
            request: Request {
                path: obj.to_owned(),
            },
            found: false,
        }
    }

    pub fn route<T, F: Fn(RouteData<T>) -> Response>
    (mut self, get_route: impl Fn(RouteData<T>) -> Route<T, F>) -> Self {
        if self.found {
            return self;
        }

        let mut route = get_route();
        if let Some(path_params) = route.matcher(&self.request.path) {
            self.found = true;
            route.enrich(RouteData::full(path_params, None));
            route.run();
        }

        self
    }
}
