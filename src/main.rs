use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/sign_in")]
async fn sign_in(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/sign_up")]
async fn sign_up(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/logout")]
async fn logout(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/:user_id")]
async fn user_id() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/:user_id/my_videos")]
async fn my_videos() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/:user_id/my_movies/:movie_id/watch")]
async fn watch() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(user_id)
            .service(user_id)
            .service(my_videos)
            .service(watch)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
