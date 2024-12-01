use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{
        ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
        QuerySelect, Set,
    },
};

use crate::security::password;

#[derive(Default, Deserialize, Serialize, Debug, ToSchema, PartialEq)]
pub enum UserIdentity {
    Administrator = 0,

    #[default]
    Normal = 1,
}

impl From<i32> for UserIdentity {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Administrator,
            _ => Self::Normal,
        }
    }
}

impl From<UserIdentity> for i32 {
    fn from(value: UserIdentity) -> Self {
        match value {
            UserIdentity::Administrator => 0,
            UserIdentity::Normal => 1,
        }
    }
}

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

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    username: String,
    password: String,
    identity: UserIdentity,
    birthday: Option<NaiveDate>,
) -> Result<user::Model, crate::error::Error> {
    if let Some(existed) = find_option_by_username(db, &username).await? {
        return Err(crate::error::Error::UserConflict(existed.username));
    }

    let hashed_password = password::hash(password)?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        password: Set(hashed_password),
        identity: Set(identity.into()),
        birthday: Set(birthday),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_user)
}
