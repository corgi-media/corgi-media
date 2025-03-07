use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{
    auth,
    types::{PasswordSignInPayload, Token},
};

use crate::{
    dto::{ErrorResponse, ResponseResult, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    request_body = PasswordSignInPayload,
    path = Paths::AUTHENTICATION_ENDPOINTS_PASSWORD,
    tag = Tags::AUTHENTICATION,
    operation_id = "auth_endpoints_password",
    summary = "Sign In (with account and password)",
    responses(
        (status = CREATED, body = Token),
        (status = UNAUTHORIZED, description = "Wrong user credentials", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    )
)]
pub async fn endpoints_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<PasswordSignInPayload>,
) -> ResponseResult<impl IntoResponse> {
    let token = auth::sessions::auth_password(
        state.database_connection(),
        &state.keyring().privite_key,
        payload.account,
        payload.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(token)))
}
