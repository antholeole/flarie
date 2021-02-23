use crate::route::Route;

struct Request {
    path: String,
}

pub struct Flarie {
    found: bool,
    request: Request,
}

impl Flarie {
    pub fn new(obj: &str) -> Self {
        Flarie {
            request: Request {
                path: obj.to_owned(),
            },
            found: false,
        }
    }

    pub fn route<T>(mut self, get_route: impl Fn() -> Route<T>) -> Self {
        if self.found {
            return self;
        }

        let route = get_route();
        if let Some(path_params) = route.matcher(&self.request.path) {
            self.found = true;
            route.enrich(Some("body".to_string()), path_params).run();
        }

        self
    }
}
