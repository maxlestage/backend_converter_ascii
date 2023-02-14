use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use sea_orm::Column;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "video")]
pub struct Video {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "user_id")]
    pub user_id: i32,
    pub titre: String,
    pub description: String,
    pub date: NaiveDate,
    pub path_to_json: String,
    pub duration: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
    #[sea_orm(belongs_to = "super::user::Entity")]
    User,
}

impl Related<super::comment::Entity> for Video {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::user::Entity> for Video {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
