use corgi_database::entities::user;

use crate::users::UserIdentity;

use super::{
    authentication::{AdminAuthentication, MixedAuthentication, UserAuthentication},
    jwt::{Audience, Claims},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Access denied: token out of range.")]
    ForbiddenAudience,

    #[error("Access denied: user is disabled.")]
    UserDisabled,

    #[error("Access denied: user is not an administrator.")]
    NotAdministrator,
}

fn is_valid_audience(aud: &Audience, allowed: &[Audience]) -> Result<(), Error> {
    if allowed.contains(aud) {
        Ok(())
    } else {
        Err(Error::ForbiddenAudience)
    }
}

pub trait Authorization {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        is_valid_audience(&claims.aud, &[Audience::User])
    }

    fn validate_user(user: &user::Model) -> Result<(), Error> {
        if user.disabled {
            Err(Error::UserDisabled)
        } else {
            Ok(())
        }
    }

    fn validate_identity(_: &user::Model) -> Result<(), Error> {
        Ok(())
    }

    fn authorize(claims: &Claims, user: &user::Model) -> Result<(), Error> {
        Self::validate_audience(claims)?;
        Self::validate_user(user)?;
        Self::validate_identity(user)?;

        Ok(())
    }
}

impl Authorization for UserAuthentication {}

impl Authorization for AdminAuthentication {
    fn validate_identity(user: &user::Model) -> Result<(), Error> {
        if UserIdentity::from(user.identity) != UserIdentity::Administrator {
            Err(Error::NotAdministrator)
        } else {
            Ok(())
        }
    }
}

impl Authorization for MixedAuthentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        is_valid_audience(&claims.aud, &[Audience::User, Audience::ApiKey])
    }
}
