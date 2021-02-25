extern crate flarie;
use flarie::{Flarie, Response};
#[macro_use]
extern crate flarie_macros;
use uuid::Uuid;


#[post("/hi/{num}/{uuid}/")]
async fn max_is_dumb((_, uuid): Path<(i32, Uuid)>) -> Response {
    let body = reqwest::get("https://www.rust-lang.org").await.unwrap().text().await.unwrap();
    println!("the num is {:?}", uuid);
    Response {}
}


fn main() {
    Flarie::new("/hi/12/936DA01F9ABD4d9d80C702AF85C822A8/", "post")
        .route(max_is_dumb);
}

