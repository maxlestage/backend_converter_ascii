use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Video::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Video::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Video::UserId).string().not_null())
                    .col(ColumnDef::new(Video::Titre).string().not_null())
                    .col(ColumnDef::new(Video::Description).string().not_null())
                    .col(ColumnDef::new(Video::Date).string().not_null())
                    .col(ColumnDef::new(Video::PathToJson).string().not_null())
                    .col(ColumnDef::new(Video::Duration).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Video::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Video {
    Table,
    Id,
    UserId,
    Titre,
    Description,
    Date,
    PathToJson,
    Duration,
}
