use corgi_database::entities::user;

use crate::users::UserIdentity;

use super::authentication::{AdminAuthentication, UserAuthentication};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Access denied: user is disabled.")]
    UserDisabled,

    #[error("Access denied: user is not an administrator.")]
    RequireAdministrator,
}

pub trait Authorization {
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

    fn authorize(user: &user::Model) -> Result<(), Error> {
        Self::validate_user(user)?;
        Self::validate_identity(user)?;

        Ok(())
    }
}

impl Authorization for UserAuthentication {}

impl Authorization for AdminAuthentication {
    fn validate_identity(user: &user::Model) -> Result<(), Error> {
        if UserIdentity::from(user.identity) != UserIdentity::Administrator {
            Err(Error::RequireAdministrator)
        } else {
            Ok(())
        }
    }
}
