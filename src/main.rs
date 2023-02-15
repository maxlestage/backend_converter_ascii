use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Insert, Statement};
use thiserror::Error;

use queries::entities;
use queries::insert_test;
#[derive(Error, Debug)]
pub enum InternalServerError {
    #[error("Task faild for JoinError: {}", 0)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Task faild for DbErr: {}", 0)]
    DbErr(#[from] DbErr),
}

pub type Result<T> = anyhow::Result<T, InternalServerError>;

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

#[get("/watch/:video_id")]
async fn watch_by_id() -> impl Responder {
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

#[get("/*")]
async fn not() -> impl Responder {
    HttpResponse::Ok().body("Hello *!")
}
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

const DATABASE_URL: &str = "postgres://postgres:codo_maton@localhost:5432";
const DB_NAME: &str = "codo_maton_db";

async fn run() -> Result<DatabaseConnection> {
    // let db = Database::connect(DATABASE_URL).await?;

    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
    // ))
    // .await?;
    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("CREATE DATABASE \"{}\";", DB_NAME),
    // ))
    // .await?;

    let url = format!("{}/{}", DATABASE_URL, DB_NAME);

    let db = Database::connect(&url).await?;
    Ok(db)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db_result = tokio::spawn(async move { run().await });

    match db_result.await {
        Ok(Ok(db)) => {
            println!("Connecté à la base de données : {}", DB_NAME);
            println!("linked to :{}", db_url);
            insert_test(db).await;
            HttpServer::new(|| {
                App::new()
                    .service(home)
                    .service(user_id)
                    .service(my_videos)
                    .service(watch)
                    .default_service(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::NotFound),
                    )
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        }
        Ok(Err(err)) => panic!("Erreur lors de la connexion à la base de données : {}", err),
        Err(err) => panic!(
            "Erreur inattendue lors de la connexion à la base de données : {}",
            err
        ),
    }
}
