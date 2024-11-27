mod account;

use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect},
};

pub use account::*;

pub async fn is_table_empty(db: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(user::Entity::find().limit(1).all(db).await?.is_empty())
}

pub async fn find_option_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<user::Model, crate::error::Error> {
    find_option_by_id(db, id)
        .await?
        .ok_or(crate::error::Error::UserNotFound)
}

pub async fn find_option_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<user::Model, crate::error::Error> {
    find_option_by_username(db, username)
        .await?
        .ok_or(crate::error::Error::UserNotFound)
}
