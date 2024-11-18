use axum::{extract::State, Json};

use crate::{
    dto::{AccountCreateRequest, ResponseResult},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

use corgi_core::{schemas::User as UserSchema, services::user};

#[utoipa::path(
    post,
    request_body = AccountCreateRequest,
    path = Paths::ACCOUNT,
    tag = Tags::ACCOUNT,
    responses(
        (status = OK, description = "Create a Account (Sign Up)", body = UserSchema)
    )
)]

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<AccountCreateRequest>,
) -> ResponseResult<Json<UserSchema>> {
    let user = user::account_create(
        state.database_connection(),
        req.name,
        req.username,
        req.password,
    )
    .await?;

    Ok(Json(user))
}
