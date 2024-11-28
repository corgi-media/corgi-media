pub mod token;

use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ActiveModelTrait, DatabaseConnection, Set},
};

use crate::{security::password, users};

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    username: String,
    password: String,
) -> Result<user::Model, crate::error::Error> {
    let is_empty = users::is_table_empty(db).await?;

    if !is_empty {
        if let Some(existed) = users::find_option_by_username(db, &username).await? {
            return Err(crate::error::Error::UserConflict(existed.username));
        }
    }
    let identity = if is_empty { 0 } else { 1 };
    let hashed_password = password::hash(password)?;

    let user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        password: Set(hashed_password),
        identity: Set(identity),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(user)
}
