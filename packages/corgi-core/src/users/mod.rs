use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{
        ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
        QueryFilter, QuerySelect, Set,
    },
};
use corgi_types::CreateUserPayload;

use crate::auth::password;

pub async fn is_table_empty(db: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(user::Entity::find().limit(1).all(db).await?.is_empty())
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<user::Model, crate::error::Error> {
    find_by_id(db, id)
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

pub async fn find_by_account(
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

pub async fn get_by_account(
    db: &DatabaseConnection,
    account: &str,
) -> Result<user::Model, crate::error::Error> {
    find_by_account(db, account)
        .await?
        .ok_or(crate::error::Error::UserNotFound)
}

pub async fn check_account_duplication(
    db: &DatabaseConnection,
    username: &str,
    email: &str,
) -> Result<(), crate::error::Error> {
    if let Some(existed) = find_by_username_or_email(db, username, email).await? {
        if existed.email == email {
            return Err(crate::error::Error::UserDuplicated(
                "email",
                existed.username,
            ));
        }
        return Err(crate::error::Error::UserDuplicated(
            "username",
            existed.username,
        ));
    }
    Ok(())
}

pub async fn create(
    db: &DatabaseConnection,
    payload: CreateUserPayload,
) -> Result<user::Model, crate::error::Error> {
    check_account_duplication(db, &payload.username, &payload.email).await?;

    let hashed_password = password::hash(payload.password).await?;

    let model = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(payload.name),
        username: Set(payload.username),
        email: Set(payload.email),
        password: Set(hashed_password),
        identity: Set(payload.identity),
        birthday: Set(payload.birthday),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(model)
}
