use crate::utils::password::PasswordError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] corgi_database::orm::DbErr),

    #[error(transparent)]
    Password(#[from] PasswordError),
}
