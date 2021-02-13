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

    pub fn get(mut self, route: &str, exe: impl Fn()) -> Self {
        if route == self.request.path {
            exe();
            self.found = true;
        }
        self
    }
}
