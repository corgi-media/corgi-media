mod account;

use corgi_database::{
    entities::user,
    orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect},
};

pub use account::*;

pub async fn is_table_empty(db: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(user::Entity::find().limit(1).all(db).await?.is_empty())
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
}
