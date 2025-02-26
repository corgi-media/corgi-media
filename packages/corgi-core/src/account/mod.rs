use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ActiveModelTrait, DatabaseConnection, Set},
};

use crate::{
    auth::password,
    users::{self, check_account_conflict, UserIdentity},
};

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    username: String,
    email: String,
    password: String,
) -> Result<user::Model, crate::error::Error> {
    let is_empty = users::is_table_empty(db).await?;

    if !is_empty {
        check_account_conflict(db, &username, &email).await?;
    }

    let identity = if is_empty {
        UserIdentity::Administrator
    } else {
        UserIdentity::Normal
    };

    let hashed_password = password::hash(password)?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        email: Set(email),
        password: Set(hashed_password),
        identity: Set(identity.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_user)
}
