use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{
    schemas::{Token, User},
    services::user,
};

use crate::{
    dto::{ErrorResponseBody, ResponseResult, SignInRequest, SignUpRequest, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    request_body = SignUpRequest,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    responses(
        (status = CREATED, description = "Create a account (Sign Up)", body = User),
        (status = CONFLICT, description = "Username conflicts", body = ErrorResponseBody),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponseBody),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<SignUpRequest>,
) -> ResponseResult<impl IntoResponse> {
    let result = user::create_account(
        state.database_connection(),
        req.name,
        req.username,
        req.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    post,
    request_body = SignInRequest,
    path = Paths::ACCOUNT_TOKEN,
    tag = Tags::ACCOUNT,
    responses(
        (status = CREATED, description = "Request token (Sign In)", body = Token),
        (status = UNAUTHORIZED, description = "Wrong user credentials", body = ErrorResponseBody),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponseBody),
    )
)]
pub async fn create_token(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<SignInRequest>,
) -> ResponseResult<impl IntoResponse> {
    let result = user::create_token(
        state.database_connection(),
        &state.config.keyring.privite_key,
        req.username,
        req.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(result)))
}
