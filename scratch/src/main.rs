extern crate flarie;
use flarie::{Flarie, Response, Route, Path, RouteData};
#[macro_use]
extern crate flarie_macros;


/*
#[post("/hi/")]
fn max_is_dumb((num, my_str): flarie::Path<(i32, string)>) -> Response {
    println!("Real resp");
    Response {}
}


fn main() {
    Flarie::new("/hi/")
        .route(max_is_dumb);
}

*/

//macro -> define the types 
//

pub fn max_is_dumb(
    data: RouteData<(i32, String)>
) -> Route<(i32, String),  fn(RouteData<(i32, String)>) -> Response> {
    fn _max_is_dumb(datai: RouteData<(i32, String)>) -> Response {
        {
            println!("Hello");
        };
        Response {}
    }

    fn _match_path_params(strings: vec<&str>) -> Option<(i32, String)> {
        if let (Some(param_str_0), Some(param_str_1)) = (strings.get(0), strings.get(1)) {
            if let (Ok(param_type_0), Ok(param_type_1)) = (param_str_0.parse::<i32>(), param_str_0.parse::<String>()) {
                (param_type_0, param_type_1)
            }
        }
        None
    }

    Route::new(_max_is_dumb, _match_path_params, "/hi/{fuck}/{big_titties}")
}

fn main() {
    Flarie::new("hi").route(max_is_dumb);
}

