extern crate flarie;
use flarie::{Flarie, Response};
#[macro_use]
extern crate flarie_macros;


/*
#[post("/hi/{nummy}/{stringmy}/")]
fn max_is_dumb((num, my_str): Path<(i32, String)>) -> Response {
    println!("the num is {}", num);
    Response {}
}
*/

fn test(test: impl std::str::FromStr) {

}


fn main() {
    Flarie::new("/hi/12/aoskd/")
        .route(max_is_dumb);
}

