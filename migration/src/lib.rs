pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230213_174341_create_table_comment;
mod m20230213_180310_create_table_video;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230213_174341_create_table_comment::Migration),
            Box::new(m20230213_180310_create_table_video::Migration),
        ]
    }
}
