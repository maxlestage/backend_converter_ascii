use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use sea_orm::Column;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comment")]
pub struct Comment {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "user_id")]
    pub user_id: i32,
    #[sea_orm(column_name = "video_id")]
    pub video_id: i32,
    pub texte: String,
    pub date: NaiveDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity")]
    User,
    #[sea_orm(belongs_to = "super::video::Entity")]
    Video,
}

impl Related<super::user::Entity> for Comment {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::video::Entity> for Comment {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
