use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use corgi_core::error::Error as CoreError;

#[derive(Debug, thiserror::Error)]
pub enum ErrorResponse {
    #[error(transparent)]
    Core(#[from] CoreError),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    Validation(#[from] garde::Report),
}

impl ErrorResponse {
    pub fn response(self) -> (StatusCode, ErrorResponseBody) {
        let (status_code, kind, message) = match &self {
            ErrorResponse::Core(error) => self.map_core_error(error),
            ErrorResponse::JsonRejection(_) => {
                (StatusCode::BAD_REQUEST, "JSON_REJECTION", self.to_string())
            }
            ErrorResponse::Validation(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "VALIDATION_FAILED",
                self.to_string(),
            ),
        };

        (status_code, ErrorResponseBody::new(kind, message))
    }

    fn map_core_error(&self, error: &CoreError) -> (StatusCode, &'static str, String) {
        match error {
            CoreError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                self.to_string(),
            ),
            CoreError::HashPassword(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "HASH_PASSWORD_ERROR",
                self.to_string(),
            ),
            CoreError::UserNotFound | CoreError::Password(_) => (
                StatusCode::UNAUTHORIZED,
                "AUTHENTICATION_FAILED",
                "Wrong user credentials".to_string(),
            ),
            CoreError::UserConflict(_) => (StatusCode::CONFLICT, "USER_CONFLICT", self.to_string()),
            CoreError::JWT(_) => (StatusCode::UNAUTHORIZED, "JWT_ERROR", self.to_string()),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct ErrorResponseBody {
    pub kind: String,
    pub message: String,
}

impl ErrorResponseBody {
    pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
        }
    }
}
