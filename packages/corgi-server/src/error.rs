use axum::{extract::rejection::JsonRejection, http::StatusCode};
use axum_extra::typed_header::TypedHeaderRejection;

use corgi_core::error::{HttpError, IntoHttpError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] corgi_core::error::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    TypedHeaderRejection(#[from] TypedHeaderRejection),

    #[error(transparent)]
    Validation(#[from] garde::Report),
}

impl IntoHttpError for Error {
    fn into_http_error(self) -> HttpError {
        match self {
            Error::Core(error) => error.into_http_error(),
            Error::JsonRejection(_) => HttpError {
                status_code: StatusCode::BAD_REQUEST,
                kind: "JSON_REJECTION",
                message: self.to_string(),
            },
            Error::TypedHeaderRejection(_) => HttpError {
                status_code: StatusCode::BAD_REQUEST,
                kind: "TYPED_HEADER_REJECTION",
                message: self.to_string(),
            },
            Error::Validation(_) => HttpError {
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                kind: "VALIDATION_FAILED",
                message: self.to_string(),
            },
        }
    }
}
