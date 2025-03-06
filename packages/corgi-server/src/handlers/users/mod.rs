use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{
    auth::authentication::AdminAuthentication,
    types::{CreateUserPayload, User},
    users,
};

use crate::{
    dto::{AuthorizedClaims, ErrorResponse, ResponseResult, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    path = Paths::USERS,
    tag = Tags::USERS,
    operation_id = "create_user",
    request_body = CreateUserPayload,
    responses(
        (status = CREATED, description = "Create a user", body = User),
        (status = CONFLICT, description = "Username conflicts", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> ResponseResult<impl IntoResponse> {
    println!("identity: {:?}", payload.identity);
    let result: User = users::create(state.database_connection(), payload)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(result)))
}
