use chrono::{Duration, Utc};

use corgi_database::{entities::user::Model as UserModel, orm::DatabaseConnection};
use corgi_types::Token;

use super::{jwt::Claims, password};
use crate::users;

pub async fn create(
    _db: &DatabaseConnection,
    privite_key: &str,
    user: &UserModel,
) -> Result<String, crate::error::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(30);

    // todo!();
    let jti = uuid::Uuid::now_v7();

    let claims = Claims {
        sub: user.id,
        jti,
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };

    let access_token = claims.encode(privite_key)?;

    Ok(access_token)
}

pub async fn auth_password(
    db: &DatabaseConnection,
    privite_key: &str,
    account: String,
    password: String,
) -> Result<Token, crate::error::Error> {
    let user = users::get_by_account(db, &account).await?;

    password::verify(&password, &user.password).await?;

    let access_token = create(db, privite_key, &user).await?;

    Ok(Token { access_token })
}
