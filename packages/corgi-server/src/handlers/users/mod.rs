use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use corgi_core::{security::authentication::AdminAuthentication, users};

use crate::{
    dto::{
        AuthorizedClaims, CreateUserRequest, ErrorResponseBody, ResponseResult, User, ValidatedJson,
    },
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    path = Paths::USERS,
    tag = Tags::USERS,
    operation_id = "create_user",
    request_body = CreateUserRequest,
    responses(
        (status = CREATED, description = "Create a user", body = User),
        (status = CONFLICT, description = "Username conflicts", body = ErrorResponseBody),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponseBody),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> ResponseResult<impl IntoResponse> {
    println!("identity: {:?}", payload.identity);
    let result: User = users::create(
        state.database_connection(),
        payload.name,
        payload.username,
        payload.password,
        payload.identity.into(),
        payload.birthday,
    )
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(result)))
}
