use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{
    schemas::{Token, User},
    services::user,
    utils::authentication::UserAuthentication,
};

use crate::{
    dto::{
        AuthorizedClaims, ErrorResponseBody, ResponseResult, SignInRequest, SignUpRequest,
        ValidatedJson,
    },
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
    ValidatedJson(payload): ValidatedJson<SignUpRequest>,
) -> ResponseResult<impl IntoResponse> {
    let result = user::create_account(
        state.database_connection(),
        payload.name,
        payload.username,
        payload.password,
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
    ValidatedJson(payload): ValidatedJson<SignInRequest>,
) -> ResponseResult<impl IntoResponse> {
    let result = user::create_token(
        state.database_connection(),
        &state.keyring().privite_key,
        payload.username,
        payload.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    get,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    responses(
        (status = OK, description = "Get account information", body = User),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn get(auth: AuthorizedClaims<UserAuthentication>) -> Json<User> {
    Json(auth.user.into())
}
