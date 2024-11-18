mod account;

use corgi_database::{
    entities::user,
    orm::{DatabaseConnection, DbErr, EntityTrait, QuerySelect},
};

pub use account::*;

pub async fn is_user_table_empty(db: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(user::Entity::find().limit(1).all(db).await?.is_empty())
}
