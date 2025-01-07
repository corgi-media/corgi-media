use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_create_library_table::Library;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(LibraryDirectory::Table)
                    .if_not_exists()
                    .col(pk_auto(LibraryDirectory::Id))
                    .col(uuid(LibraryDirectory::LibraryId))
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_library_directory_library_id")
                            .from(LibraryDirectory::Table, LibraryDirectory::LibraryId)
                            .to(Library::Table, Library::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(LibraryDirectory::Path))
                    .col(timestamp_null(LibraryDirectory::ScannedAt))
                    .to_owned(),
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LibraryDirectory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LibraryDirectory {
    Table,
    Id,
    LibraryId,
    Path,
    ScannedAt,
}
