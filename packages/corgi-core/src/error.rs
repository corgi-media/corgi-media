use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] corgi_database::orm::DbErr),

    #[error("{0}")]
    HashPassword(String),

    #[error("{0}")]
    Password(String),

    #[error("User duplicated: {0} `{1}` already exists.")]
    UserDuplicated(&'static str, String),

    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Authorization(#[from] crate::auth::authorization::Error),

    #[error("{0} not found")]
    NotFound(&'static str),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Library directory duplicated: {0}")]
    LibraryDirectoryDuplicated(String),

    #[error("Sign Up is disabled")]
    SignUpDisabled,
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        match value {
            argon2::password_hash::Error::Password => Error::Password(value.to_string()),
            _ => Error::HashPassword(value.to_string()),
        }
    }
}

pub struct HttpError {
    pub status_code: StatusCode,
    pub kind: &'static str,
    pub message: String,
}

pub trait IntoHttpError {
    fn into_http_error(self) -> HttpError;
}

impl IntoHttpError for Error {
    fn into_http_error(self) -> HttpError {
        match self {
            Error::Database(_) => HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                kind: "DATABASE_ERROR",
                message: self.to_string(),
            },
            Error::HashPassword(_) => HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                kind: "HASH_PASSWORD_ERROR",
                message: self.to_string(),
            },
            Error::UserNotFound | Error::Password(_) => HttpError {
                status_code: StatusCode::UNAUTHORIZED,
                kind: "AUTHENTICATION_FAILED",
                message: "Wrong user credentials".to_string(),
            },
            Error::UserDuplicated(_, _) => HttpError {
                status_code: StatusCode::CONFLICT,
                kind: "USER_DUPLICATED",
                message: self.to_string(),
            },
            Error::JWT(_) => HttpError {
                status_code: StatusCode::UNAUTHORIZED,
                kind: "JWT_ERROR",
                message: self.to_string(),
            },
            Error::Authorization(_) => HttpError {
                status_code: StatusCode::FORBIDDEN,
                kind: "AUTHORIZATION_ERROR",
                message: self.to_string(),
            },
            Error::NotFound(_) => HttpError {
                status_code: StatusCode::NOT_FOUND,
                kind: "NOT_FOUND",
                message: self.to_string(),
            },
            Error::InvalidPath(_) => HttpError {
                status_code: StatusCode::BAD_REQUEST,
                kind: "INVALID_PATH",
                message: self.to_string(),
            },
            Error::LibraryDirectoryDuplicated(_) => HttpError {
                status_code: StatusCode::CONFLICT,
                kind: "LIBRARY_DIRECTORY_DUPLICATED",
                message: self.to_string(),
            },
            Error::SignUpDisabled => HttpError {
                status_code: StatusCode::FORBIDDEN,
                kind: "SIGN_UP_DISABLED",
                message: self.to_string(),
            },
        }
    }
}
