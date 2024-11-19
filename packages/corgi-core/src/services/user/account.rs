use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel},
};

use crate::schemas::{User as UserSchema, UserIdentity};

pub async fn create_account(
    db: &DatabaseConnection,
    name: String,
    username: String,
    password: String,
) -> Result<UserSchema, crate::error::Error> {
    let is_empty = super::is_table_empty(db).await?;

    if !is_empty {
        if let Some(existed) = super::find_by_username(db, &username).await? {
            return Err(crate::error::Error::UserConflict(existed.username));
        }
    }
    let identity = if is_empty {
        UserIdentity::Administrator
    } else {
        UserIdentity::Normal
    };
    let hashed_password = crate::utils::password::hash(password)?;

    let user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        password: Set(hashed_password),
        identity: Set(identity.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let model = user.try_into_model()?;

    Ok(UserSchema::from(model))
}
