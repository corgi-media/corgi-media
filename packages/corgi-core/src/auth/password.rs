use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("{0}")]
    Hash(String),

    #[error("{0}")]
    Password(String),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
}

impl From<argon2::password_hash::Error> for PasswordError {
    fn from(value: argon2::password_hash::Error) -> Self {
        match value {
            argon2::password_hash::Error::Password => PasswordError::Password(value.to_string()),
            _ => PasswordError::Hash(value.to_string()),
        }
    }
}
impl crate::error::IntoHttpError for PasswordError {
    fn into_http_error(self) -> crate::error::HttpError {
        match self {
            PasswordError::Hash(_) => crate::error::HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                kind: "HASH_PASSWORD_ERROR",
                message: self.to_string(),
            },
            PasswordError::Password(_) => crate::error::HttpError {
                status_code: StatusCode::UNAUTHORIZED,
                kind: "AUTHENTICATION_FAILED",
                message: "Wrong user credentials".to_string(),
            },
            PasswordError::Join(_) => crate::error::HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                kind: "TOKIO_JOIN_ERROR",
                message: self.to_string(),
            },
        }
    }
}

fn hash_password(password: impl AsRef<str>) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_ref().as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn verify_password(
    password: impl AsRef<str>,
    hash: impl AsRef<str>,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash.as_ref())?;

    let argon2 = Argon2::default();

    argon2.verify_password(password.as_ref().as_bytes(), &parsed_hash)
}

pub async fn hash(password: impl AsRef<str>) -> Result<String, PasswordError> {
    let password = password.as_ref().to_string();
    let hash = tokio::task::spawn_blocking(move || hash_password(password)).await??;

    Ok(hash)
}

pub async fn verify(password: impl AsRef<str>, hash: impl AsRef<str>) -> Result<(), PasswordError> {
    let password = password.as_ref().to_string();
    let hash = hash.as_ref().to_string();
    tokio::task::spawn_blocking(move || verify_password(password, hash)).await??;

    Ok(())
}
