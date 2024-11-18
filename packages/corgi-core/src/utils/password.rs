use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("{0}")]
    Hash(String),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
}

impl From<argon2::password_hash::Error> for PasswordError {
    fn from(value: argon2::password_hash::Error) -> Self {
        PasswordError::Hash(value.to_string())
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

pub async fn hash(password: String) -> Result<String, PasswordError> {
    let hash = tokio::task::spawn_blocking(move || hash_password(password)).await??;

    Ok(hash)
}

pub async fn verify(password: String, hash: String) -> Result<(), PasswordError> {
    tokio::task::spawn_blocking(move || verify_password(password, hash)).await??;

    Ok(())
}
