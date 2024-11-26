use corgi_database::entities::user;

use super::{
    authentication::{AdminAuthentication, MixedAuthentication, UserAuthentication},
    claims::{Audience, Claims},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Access denied: token out of range.")]
    ForbiddenAudience,

    #[error("Access denied: user is disabled.")]
    UserDisabled,
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

    fn authorize(claims: &Claims, user: &user::Model) -> Result<(), Error> {
        Self::validate_audience(claims)?;
        Self::validate_user(user)?;

        Ok(())
    }
}

impl Authorization for UserAuthentication {}

impl Authorization for AdminAuthentication {}

impl Authorization for MixedAuthentication {
    fn validate_audience(claims: &Claims) -> Result<(), Error> {
        is_valid_audience(&claims.aud, &[Audience::User, Audience::ApiKey])
    }
}
