use corgi_database::{entities::user, orm::DatabaseConnection};

use super::claims::{Audience, Claims};
use crate::{error::Error, services::user::find_by_id};

pub trait Authentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error>;

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

impl Authentication for UserAuthentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        if matches!(claims.aud, Audience::User) {
            Ok(())
        } else {
            Err(Error::JWT(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidAudience,
            )))
        }
    }
}

pub struct MixedAuthentication;

impl Authentication for MixedAuthentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        if matches!(claims.aud, Audience::User | Audience::ApiKey) {
            Ok(())
        } else {
            Err(Error::JWT(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidAudience,
            )))
        }
    }
}
