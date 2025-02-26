use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string_len(User::Name, 64))
                    .col(string_len_uniq(User::Username, 64))
                    .col(string_len_uniq(User::Email, 254))
                    .col(string_len(User::Password, 128))
                    .col(integer(User::Identity).default(1).take())
                    .col(date_null(User::Birthday))
                    .col(timestamp_null(User::LockedUntil))
                    .col(boolean(User::Disabled).default(false).take())
                    .col(timestamp_null(User::DisabledAt))
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
    Email,
    Password,
    Identity,
    Birthday,
    LockedUntil,
    Disabled,
    DisabledAt,
}
