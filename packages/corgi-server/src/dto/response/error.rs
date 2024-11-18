use axum::{
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
}

impl ErrorResponse {
    pub fn response(self) -> (StatusCode, ErrorResponseBody) {
        let message = self.to_string();
        let (status_code, kind) = match self {
            ErrorResponse::Core(error) => match error {
                CoreError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
                CoreError::Password(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "PASSWORD_HASH_ERROR")
                }
            },
        };

        (status_code, ErrorResponseBody::new(kind, message))
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
