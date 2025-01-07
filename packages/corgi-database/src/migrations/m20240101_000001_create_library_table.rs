use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(Library::Table)
                    .if_not_exists()
                    .col(pk_uuid(Library::Id))
                    .col(string_len_uniq(Library::Name, 128))
                    .col(string_len(Library::Category, 32))
                    .col(string_len(Library::Language, 2))
                    .col(string_len(Library::Region, 2))
                    .col(string_len_null(Library::MetadataProvider, 64))
                    .col(timestamp_null(Library::ScannedAt))
                    .to_owned(),
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Library::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Library {
    Table,
    Id,
    Name,
    Category,
    // ISO 639-1
    Language,
    // ISO 3166-1 alpha-2
    Region,
    MetadataProvider,
    ScannedAt,
}
