extern crate flarie;
use flarie::{Flarie, Response, Route, Path};
#[macro_use]
extern crate flarie_macros;

/*
#[post("/hi/{fuck}/{big_titties}")]
fn max_is_dumb((num, my_str): flarie::Path<(i32, string)>) -> Response {
    println!("Real resp");
    Response {}
}


fn main() {
    Flarie::new("hi")
        .route(max_is_dumb);
}
*/

pub fn max_is_dumb() -> Route {
    fn _max_is_dumb() -> Response {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["Real resp\n"],
                &match () {
                    () => [],
                },
            ));
        };
        Response {}
    }
    Route::new(_max_is_dumb, "/hi/{fuck}/{big_titties}")
}

fn main() {
    Flarie::new("hi").route(max_is_dumb);
}
