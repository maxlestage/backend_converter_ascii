use chrono::NaiveDate;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct User {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub Firstname: String,
    pub Lastname: String,
    pub SignUpDate: NaiveDate,
    pub Mail: String,
    pub Password: String,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::fruit::Entity")]
//     Fruit,
// }

// impl Related<super::fruit::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Fruit.def()
//     }
// }

impl ActiveModelBehavior for User {}
