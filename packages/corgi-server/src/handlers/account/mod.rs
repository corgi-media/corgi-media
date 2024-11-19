use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{schemas::User as UserSchema, services::user};

use crate::{
    dto::{AccountCreateRequest, ErrorResponseBody, ResponseResult},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    request_body = AccountCreateRequest,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    responses(
        (status = CREATED, description = "Create a Account (Sign Up)", body = UserSchema),
        (status = CONFLICT, description = "Username conflicts", body = ErrorResponseBody)
    )
)]

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<AccountCreateRequest>,
) -> ResponseResult<impl IntoResponse> {
    let user = user::account_create(
        state.database_connection(),
        req.name,
        req.username,
        req.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}
