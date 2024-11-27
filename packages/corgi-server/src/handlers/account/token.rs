use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{schemas::Token, services::user};

use crate::{
    dto::{ErrorResponseBody, ResponseResult, SignInRequest, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

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
pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SignInRequest>,
) -> ResponseResult<impl IntoResponse> {
    let result = user::account::create_token(
        state.database_connection(),
        &state.keyring().privite_key,
        payload.username,
        payload.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(result)))
}
