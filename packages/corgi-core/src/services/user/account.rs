use uuid::Uuid;

use corgi_database::{
    entities::user,
    orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel},
};

use crate::schemas::User as UserSchema;

pub async fn account_create(
    db: &DatabaseConnection,
    name: String,
    username: String,
    password: String,
) -> Result<UserSchema, crate::error::Error> {
    let user_empty = super::is_user_table_empty(db).await?;
    let password_hash = crate::utils::password::hash(password).await?;

    let user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        username: Set(username),
        password: Set(password_hash),
        administrator: Set(user_empty),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let model = user.try_into_model()?;

    Ok(UserSchema::from(model))
}
