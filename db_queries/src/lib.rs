use chrono::prelude::*;
use chrono::NaiveDate;
mod entities;

const DATE: NaiveDate = NaiveDate::from(Local::now()).unwrap();
async fn insert() {
    let user = user::ActiveModel {
        firstname: Set("John".to_owned()),
        lastname: Set("Doe".to_owned()),
        signup_date: Set(DATE.format("%Y-%m-%d").to_owned()),
        mail: Set("johndoe@mail.com".to_owned()),
        password: Set("azerty".to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };

    let user: user::Model = user.insert(db).await?;
}

async fn main() {
    insert()
}
