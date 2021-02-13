mod flarie;
mod routes;

fn hello() -> String {
    return "hi".to_owned();
}
/*

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

(obj: JsValue) {
    Flarie
        .service(web::scope("/app").route("/index.html", web::get().to(index)))
        .service(web::scope("/").guard(guard::Header("Bearer-")))
    })


Flarie::run(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
}
*/
