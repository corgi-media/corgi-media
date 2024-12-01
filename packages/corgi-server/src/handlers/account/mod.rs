pub mod token;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{account, security::authentication::UserAuthentication};

use crate::{
    dto::{
        AuthorizedClaims, ErrorResponseBody, ResponseResult, SignUpRequest, User, ValidatedJson,
    },
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    operation_id = "sign_up",
    request_body = SignUpRequest,
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
    let result: User = account::create(
        state.database_connection(),
        payload.name,
        payload.username,
        payload.password,
    )
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    get,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    operation_id = "account_information",
    responses(
        (status = OK, description = "Get account information", body = User),
        (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponseBody),
        (status = FORBIDDEN, description = "Forbidden", body = ErrorResponseBody),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn get(auth: AuthorizedClaims<UserAuthentication>) -> Json<User> {
    Json(auth.user.into())
}
