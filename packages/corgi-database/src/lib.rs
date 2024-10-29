pub use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub struct DatabaseClient {
    pub connection: DatabaseConnection,
}

impl DatabaseClient {
    pub async fn connect(url: &str) -> Result<DatabaseClient, DbErr> {
        let options = ConnectOptions::new(url);

        let connection = Database::connect(options).await?;

        Ok(Self { connection })
    }
}
