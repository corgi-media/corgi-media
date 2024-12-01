use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::account;

use crate::{
    dto::{ErrorResponseBody, ResponseResult, SignInRequest, Token, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    request_body = SignInRequest,
    path = Paths::ACCOUNT_TOKEN,
    tag = Tags::ACCOUNT,
    operation_id = "sign_in",
    responses(
        (status = CREATED, description = "Request token (Sign In)", body = Token),
        (status = UNAUTHORIZED, description = "Wrong user credentials", body = ErrorResponseBody),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponseBody),
    )
)]
pub async fn create_token(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SignInRequest>,
) -> ResponseResult<impl IntoResponse> {
    let access_token = account::token::create(
        state.database_connection(),
        &state.keyring().privite_key,
        payload.username,
        payload.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(Token { access_token })))
}
