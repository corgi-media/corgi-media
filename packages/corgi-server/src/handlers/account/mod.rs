pub mod token;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{
    account,
    auth::authentication::UserAuthentication,
    types::{SignUpPayload, User},
};

use crate::{
    dto::{AuthorizedClaims, ErrorResponse, ResponseResult, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    operation_id = "sign_up",
    request_body = SignUpPayload,
    responses(
        (status = CREATED, description = "Create a account (Sign Up)", body = User),
        (status = CONFLICT, description = "Username conflicts", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SignUpPayload>,
) -> ResponseResult<impl IntoResponse> {
    let result: User = account::create(state.database_connection(), payload)
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
        (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
        (status = FORBIDDEN, description = "Forbidden", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn get(auth: AuthorizedClaims<UserAuthentication>) -> Json<User> {
    Json(auth.user.into())
}
