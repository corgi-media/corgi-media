use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{
        ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
        QueryFilter, QuerySelect, Set,
    },
};

use crate::auth::password;

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

pub async fn find_by_id_option(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<user::Model, crate::error::Error> {
    find_by_id_option(db, id)
        .await?
        .ok_or(crate::error::Error::UserNotFound)
}

pub async fn find_by_username_or_email(
    db: &DatabaseConnection,
    username: &str,
    email: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(username))
                .add(user::Column::Email.eq(email)),
        )
        .one(db)
        .await
}

pub async fn find_by_account_option(
    db: &DatabaseConnection,
    account: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Email.eq(account))
                .add(user::Column::Username.eq(account)),
        )
        .one(db)
        .await
}

pub async fn find_by_account(
    db: &DatabaseConnection,
    account: &str,
) -> Result<user::Model, crate::error::Error> {
    find_by_account_option(db, account)
        .await?
        .ok_or(crate::error::Error::UserNotFound)
}

pub async fn check_account_conflict(
    db: &DatabaseConnection,
    username: &str,
    email: &str,
) -> Result<(), crate::error::Error> {
    if let Some(existed) = find_by_username_or_email(db, username, email).await? {
        if existed.email == email {
            return Err(crate::error::Error::UserConflict("email", existed.username));
        }
        return Err(crate::error::Error::UserConflict(
            "username",
            existed.username,
        ));
    }
    Ok(())
}

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    username: String,
    email: String,
    password: String,
    identity: UserIdentity,
    birthday: Option<NaiveDate>,
) -> Result<user::Model, crate::error::Error> {
    check_account_conflict(db, &username, &email).await?;

    let hashed_password = password::hash(password)?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        email: Set(email),
        password: Set(hashed_password),
        identity: Set(identity.into()),
        birthday: Set(birthday),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_user)
}
