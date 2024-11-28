use corgi_database::orm::DatabaseConnection;

use crate::{
    security::{
        jwt::{Audience, Claims},
        password,
    },
    users,
};

pub async fn create(
    db: &DatabaseConnection,
    privite_key: &str,
    username: String,
    password: String,
) -> Result<String, crate::error::Error> {
    let user = users::find_by_username(db, &username).await?;

    password::verify(&password, &user.password)?;

    let claims = Claims::new(Audience::User, user.id, user.username, 30);

    let access_token = claims.encode(privite_key)?;

    Ok(access_token)
}
