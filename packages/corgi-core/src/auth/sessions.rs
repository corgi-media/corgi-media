use corgi_database::orm::DatabaseConnection;

use crate::users;

use super::{jwt::Claims, password};

pub async fn auth_password(
    db: &DatabaseConnection,
    privite_key: &str,
    account: String,
    password: String,
) -> Result<String, crate::error::Error> {
    let user = users::find_by_account(db, &account).await?;

    password::verify(&password, &user.password)?;

    let claims = Claims::new(user.id, 30);

    let access_token = claims.encode(privite_key)?;

    Ok(access_token)
}
