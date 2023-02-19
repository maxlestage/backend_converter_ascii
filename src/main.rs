use actix_web::delete;
use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use entities::user;
// use futures::executor::block_on;::
// use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Insert, Statement};
use entities::prelude::*;
use queries::*;
use sea_orm::ActiveModelTrait;
use sea_orm::JsonValue;
use sea_orm::{Database, DatabaseConnection, DbErr};
use thiserror::Error;

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

#[post("/logout")]
async fn logout(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/test")]
// async fn test() -> impl Responder {
//     let db_result = tokio::spawn(async move { run().await });

//     match db_result.await {
//         Ok(Ok(db)) => {
//             insert_test(db).await;
//         }
//         Ok(Err(err)) => panic!("Erreur lors de la connexion à la base de données : {}", err),
//         Err(err) => panic!(
//             "Erreur inattendue lors de la connexion à la base de données : {}",
//             err
//         ),
//     }
//     HttpResponse::Ok().body("Hello world!")
// }

#[post("/sign_up")]
async fn sign_up(user_input: web::Json<JsonValue>) -> impl Responder {
    let db_result = tokio::spawn(async move { run().await });

    let user: user::Model;
    let Ok(user_active_model) = user::ActiveModel::from_json(user_input.into_inner()) else {
  return HttpResponse::NotAcceptable().finish();
};

    match db_result.await {
        Ok(Ok(db)) => {
            if let Some(user_check) = insert_test(db, user_active_model).await {
                user = user_check;
            } else {
                return HttpResponse::Conflict().finish();
            }
        }
        Ok(Err(err)) => panic!("Erreur lors de la connexion à la base de données : {}", err),
        Err(err) => panic!(
            "Erreur inattendue lors de la connexion à la base de données : {}",
            err
        ),
    }

    HttpResponse::Created().json(user)
}

#[delete("/delete")]
async fn delete(id: web::Json<JsonValue>) -> impl Responder {
    let db_result = tokio::spawn(async move { run().await });

    let Some(id) = id["id"].as_i64()  else {
  return HttpResponse::NotAcceptable().finish();
};

    match db_result.await {
        Ok(Ok(db)) => {
            delete_test(db, id as i32).await;
        }
        Ok(Err(err)) => panic!("Erreur lors de la connexion à la base de données : {}", err),
        Err(err) => panic!(
            "Erreur inattendue lors de la connexion à la base de données : {}",
            err
        ),
    }
    HttpResponse::Ok().finish()
}

#[get("/select/{id}")]
async fn select(id: web::Path<(i32,)>) -> impl Responder {
    let db_result = tokio::spawn(async move { run().await });

    match db_result.await {
        Ok(Ok(db)) => {
            select_test(db, id.into_inner().0).await;
        }
        Ok(Err(err)) => panic!("Erreur lors de la connexion à la base de données : {}", err),
        Err(err) => panic!(
            "Erreur inattendue lors de la connexion à la base de données : {}",
            err
        ),
    }
    HttpResponse::Ok().body("Hello world!")
}

#[get("/watch/{video_id}")]
async fn watch_by_id() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{user_id}")]
async fn user_id() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{user_id}/my_videos")]
async fn my_videos() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{user_id}/my_movies/{movie_id}/watch")]
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

    let url: String = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;
    Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db_result = tokio::spawn(async move { run().await });

    match db_result.await {
        Ok(Ok(_db)) => {
            println!("Connecté à la base de données : {}", DB_NAME);
            println!("linked to :{}", db_url);
            // insert_test(db).await;
            HttpServer::new(|| {
                App::new()
                    .service(home)
                    .service(user_id)
                    .service(my_videos)
                    .service(watch)
                    // .service(test)//
                    .service(delete)
                    .service(select)
                    .default_service(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::NotFound),
                    )
            })
            .bind(("0.0.0.0", 8080))?
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
