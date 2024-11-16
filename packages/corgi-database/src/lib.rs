pub mod entities;
pub mod migrations;

pub use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use migrations::{Migrator, MigratorTrait};

pub struct DatabaseClient {
    pub connection: DatabaseConnection,
}

impl DatabaseClient {
    pub async fn connect(url: &str) -> Result<DatabaseClient, DbErr> {
        let options = ConnectOptions::new(url);

        let connection = Database::connect(options).await?;

        Ok(Self { connection })
    }

    pub async fn migration_up(&self) -> Result<(), DbErr> {
        Migrator::up(&self.connection, None).await
    }

    pub async fn migration_down(&self) -> Result<(), DbErr> {
        Migrator::down(&self.connection, None).await
    }
}
