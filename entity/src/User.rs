use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use sea_orm::Column;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct User {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub signup_date: NaiveDate,
    pub mail: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::video::Entity")]
    Videos,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
}

impl Related<super::video::Entity> for User {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl Related<super::comment::Entity> for User {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
