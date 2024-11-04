use sea_orm_migration::{prelude::*, schema::*};

use super::general::general_table_cols;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(general_table_cols(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string_len(User::Name, 0x40))
                    .col(string_len_uniq(User::Username, 0x40))
                    .col(string_len(User::Password, 0x80))
                    .col(boolean(User::Administrator).default(false).take())
                    .col(date_null(User::Birthday))
                    .col(date_time_null(User::LastLoginAt))
                    .col(date_time_null(User::LastActivityAt))
                    .col(date_time_null(User::LockedUntil))
                    .col(boolean(User::Disabled).default(false).take())
                    .col(date_time_null(User::DisabledAt))
                    .to_owned(),
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Username,
    Password,
    Administrator,
    Birthday,
    LastLoginAt,
    LastActivityAt,
    LockedUntil,
    Disabled,
    DisabledAt,
}
