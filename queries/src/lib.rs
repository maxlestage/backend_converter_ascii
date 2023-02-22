// use chrono::prelude::*;/
use chrono::Local;
use entities::prelude::*;
use entities::*;
// use futures::future::ok;
// use entities::user::ActiveModel;
// use chrono::NaiveDate;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
// use sea_orm::Set;
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Error, Value};

use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn create_user(
    db: DatabaseConnection,
    user_input: user::ActiveModel,
) -> Option<user::Model> {
    let mut user_inputed = user_input;

    let user_password = user_inputed.password.as_ref().clone();
    // println!("{}", user_password);
    let hashed = hash(&user_password, DEFAULT_COST).unwrap();
    // println!("{}", hashed);
    // let valid = verify(&user_password, &hashed).unwrap();
    // println!("{}", valid);

    user_inputed.sign_up_date =
        sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));

    user_inputed.password = sea_orm::ActiveValue::Set(hashed);

    let user_mail = user_inputed.mail.as_ref().clone();
    if User::find()
        .filter(user::Column::Mail.eq(user_mail))
        .one(&db)
        .await
        .expect("pas gérée")
        .is_some()
    {
        return None;
    }

    let user: user::Model = user_inputed.insert(&db).await.expect("Insertion loupé");
    Some(user)
}

pub async fn delete_user_by_id(db: DatabaseConnection, id: i32) {
    let deleted: DeleteResult = User::delete_by_id(id)
        .exec(&db)
        .await
        .expect("Deletion loupé");
    dbg!(deleted);
}

pub async fn select_user_by_id(db: DatabaseConnection, id: i32) {
    let _select_user: Option<user::Model> =
        User::find_by_id(id).one(&db).await.expect("Select loupé");
    // dbg!(select_user);
}

pub async fn select_user_by_email(db: DatabaseConnection, user_inputed: user::ActiveModel) -> bool {
    let user_mail = user_inputed.mail.as_ref().clone();
    if User::find()
        .filter(user::Column::Mail.eq(user_mail))
        .one(&db)
        .await
        .expect("pas gérée")
        .is_some()
    {
        true
    } else {
        false
    }
}

pub async fn register_user(
    db: DatabaseConnection,
    user_input: user::ActiveModel,
) -> Option<user::Model> {
    let user_inputed = user_input;

    let user_password = user_inputed.password.as_ref().clone();
    // println!("{}", user_password);
    let hashed = hash(&user_password, DEFAULT_COST).unwrap();
    // println!("{}", hashed);
    let _valid = verify(&user_password, &hashed).unwrap();
    // println!("{}", valid);

    // user_inputed.password = sea_orm::ActiveValue::Set(hashed);

    let user_mail = user_inputed.mail.as_ref().clone();

    let select_user: Option<user::Model> = User::find()
        .filter(user::Column::Mail.eq(user_mail))
        .one(&db)
        .await
        .expect("Select loupé");
    dbg!(select_user)
}

pub async fn password_is_valid(db: DatabaseConnection, user_input: user::ActiveModel) -> bool {
    // println!("{:#?}", &db);
    let select_user: Option<user::Model> = User::find()
        .filter(user::Column::Password.eq(&user_input.password))
        .one(&db)
        .await
        .expect("Not Found");
    // dbg!(&select_user);

    println!("Je test select_user : {:#?}", select_user);

    let user_password = select_user.clone().map(|user| user.password).unwrap();
    println!("is user_password: {}", user_password);
    let hashed = hash(&user_input.password, DEFAULT_COST).unwrap();
    println!("is hashed: {}", hashed);
    let valid = verify(&user_password, &hashed).unwrap();
    println!("is valid: {}", valid);

    // user_inputed.password = sea_orm::ActiveValue::Set(hashed);

    if valid {
        true
    } else {
        false
    }
}
