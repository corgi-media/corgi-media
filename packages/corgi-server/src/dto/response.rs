use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use corgi_core::error::IntoHttpError;

use crate::error::Error;

pub type ResponseResult<T = ()> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let http_error = self.into_http_error();

        (
            http_error.status_code,
            Json(ErrorResponse {
                kind: http_error.kind,
                message: http_error.message,
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct ErrorResponse {
    pub kind: &'static str,
    pub message: String,
}
