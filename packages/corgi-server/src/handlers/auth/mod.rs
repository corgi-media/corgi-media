use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::auth;

use crate::{
    dto::{ErrorResponseBody, PasswordSignInRequest, ResponseResult, Token, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    request_body = PasswordSignInRequest,
    path = Paths::AUTHENTICATION_ENDPOINTS_PASSWORD,
    tag = Tags::AUTHENTICATION,
    operation_id = "auth_endpoints_password",
    responses(
        (status = CREATED, description = "Sign In (use account and password)", body = Token),
        (status = UNAUTHORIZED, description = "Wrong user credentials", body = ErrorResponseBody),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponseBody),
    )
)]
pub async fn endpoints_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<PasswordSignInRequest>,
) -> ResponseResult<impl IntoResponse> {
    let access_token = auth::sessions::auth_password(
        state.database_connection(),
        &state.keyring().privite_key,
        payload.account,
        payload.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(Token { access_token })))
}
