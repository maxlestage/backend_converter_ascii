use chrono::prelude::*;
use entities::prelude::*;
use entities::*;
// use futures::future::ok;
use entities::user::ActiveModel;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

pub async fn insert_test(
    db: DatabaseConnection,
    user_input: user::ActiveModel,
) -> Option<user::Model> {
    // Find by primary key

    let user_mail = user_input.mail.as_ref().clone();

    if User::find()
        .filter(user::Column::Mail.eq(user_mail))
        .one(&db)
        .await
        .expect("pas gérer")
        .is_some()
    {
        return None;
    }

    let user: user::Model = user_input.insert(&db).await.expect("Insertion loupé");
    Some(user)
}

pub async fn delete_test(db: DatabaseConnection, id: i32) {
    let deleted: DeleteResult = User::delete_by_id(id)
        .exec(&db)
        .await
        .expect("Deletion loupé");
    dbg!(deleted);
}

pub async fn select_test(db: DatabaseConnection, id: i32) {
    // Find by primary key
    let select_user: Option<user::Model> =
        User::find_by_id(id).one(&db).await.expect("Select loupé");
    dbg!(select_user);
}
