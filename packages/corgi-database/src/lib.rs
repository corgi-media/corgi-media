pub mod entities;
pub mod migrations;

pub use sea_orm as orm;

use migrations::{Migrator, MigratorTrait};

pub struct DatabaseClient {
    pub connection: orm::DatabaseConnection,
}

impl DatabaseClient {
    pub async fn connect(url: &str) -> Result<DatabaseClient, orm::DbErr> {
        let options = orm::ConnectOptions::new(url);

        let connection = orm::Database::connect(options).await?;

        Ok(Self { connection })
    }

    pub async fn migration_up(&self) -> Result<(), orm::DbErr> {
        Migrator::up(&self.connection, None).await
    }

    pub async fn migration_down(&self) -> Result<(), orm::DbErr> {
        Migrator::down(&self.connection, None).await
    }
}
