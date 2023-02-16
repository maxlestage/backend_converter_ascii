use chrono::prelude::*;
use entities::prelude::*;
use entities::*;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::Set;

// const DATE: NaiveDate = NaiveDate::from(Local::now()).unwrap();
pub async fn insert_test(db: DatabaseConnection) {
    let user = user::ActiveModel {
        firstname: Set("John".to_owned()),
        lastname: Set("Doe".to_owned()),
        sign_up_date: Set(Local::now().date_naive().to_owned()),
        mail: Set("johndoe@mail.com".to_owned()),
        password: Set("azerty".to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };

    let user: user::Model = user.insert(&db).await.expect("Insertion loupé");
    dbg!(user);
}

pub async fn delete_test(db: DatabaseConnection, id: i32) {
    let _res: DeleteResult = User::delete_by_id(id)
        .exec(&db)
        .await
        .expect("Deletion loupé");
}
