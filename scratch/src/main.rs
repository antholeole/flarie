extern crate flarie;
use flarie::{Flarie, Response, Route, Path, RouteData};
#[macro_use]
extern crate flarie_macros;



#[post("/hi/{int}/{str}")]
fn max_is_dumb((num, my_str): flarie::Path<(i32, string)>) -> Response {
    Response {}
}

fn main() {
    Flarie::new("/hi/123/")
        .route(max_is_dumb);

    return "hi";
}


//macro -> define the types 
//


/*
pub fn max_is_dumb() -> Route<(i32, String)> {
    fn _max_is_dumb(datai: RouteData<(i32, String)>) -> Response {
        {
            println!("{:?}", datai.path_params);
        };
        Response {}
    }

    use flarie::_match_2_path_params;
    Route::new(_max_is_dumb, _match_2_path_params::<i32, String>, "/hi/{fuck}/{big_titties}")
}


fn main() {
    Flarie::new("hi").route(max_is_dumb);
}
*/
