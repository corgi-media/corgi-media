use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ActiveModelTrait, DatabaseConnection, Set},
};
use corgi_types::{SignUpPayload, UserIdentity};

use crate::{auth::password, users};

pub async fn create(
    db: &DatabaseConnection,
    payload: SignUpPayload,
) -> Result<user::Model, crate::error::Error> {
    let is_empty = users::is_table_empty(db).await?;

    if !is_empty {
        // check_account_duplication(db, &payload.username, &payload.email).await?;
        return Err(crate::error::Error::SignUpDisabled);
    }

    let identity = if is_empty {
        UserIdentity::Administrator
    } else {
        UserIdentity::Normal
    };

    let hashed_password = password::hash(payload.password)?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(payload.name),
        username: Set(payload.username),
        email: Set(payload.email),
        password: Set(hashed_password),
        identity: Set(identity.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_user)
}
