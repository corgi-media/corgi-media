use corgi_database::{entities::user, orm::DatabaseConnection};

use super::claims::{Audience, Claims};
use crate::{error::Error, services::user::find_by_id};

fn is_valid_audience(aud: &Audience, allowed: &[Audience]) -> Result<(), Error> {
    if allowed.contains(aud) {
        Ok(())
    } else {
        Err(Error::JWT(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidAudience,
        )))
    }
}

pub trait Authentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        is_valid_audience(&claims.aud, &[Audience::User])
    }

    fn validate_user(
        db: &DatabaseConnection,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<user::Model, Error>> + Send {
        async { find_by_id(db, claims.iss).await }
    }

    fn authenticate(
        db: &DatabaseConnection,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<user::Model, Error>> + Send {
        async {
            Self::validate_audience(claims)?;
            Self::validate_user(db, claims).await
        }
    }
}

pub struct UserAuthentication;

impl Authentication for UserAuthentication {}

pub struct MixedAuthentication;

impl Authentication for MixedAuthentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        is_valid_audience(&claims.aud, &[Audience::User, Audience::ApiKey])
    }
}
