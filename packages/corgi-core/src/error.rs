#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] corgi_database::orm::DbErr),

    #[error("{0}")]
    HashPassword(String),

    #[error("{0}")]
    Password(String),

    #[error("Username `{0}` is already taken")]
    UserConflict(String),

    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        match value {
            argon2::password_hash::Error::Password => Error::Password(value.to_string()),
            _ => Error::HashPassword(value.to_string()),
        }
    }
}
