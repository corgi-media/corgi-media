pub mod directories;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use corgi_core::{
    auth::authentication::{AdminAuthentication, UserAuthentication},
    error::Error as CoreError,
    libraries,
    types::{Library, LibraryPayload, Paginated, Pagination},
    uuid::Uuid,
};

use crate::{
    dto::{AuthorizedClaims, ErrorResponse, ResponseResult, ValidatedJson},
    openapi::Tags,
    routers::Paths,
    state::AppState,
};

#[utoipa::path(
    post,
    path = Paths::LIBRARIES,
    tag = Tags::LIBRARIES,
    operation_id = "create_library",
    request_body = LibraryPayload,
    responses(
        (status = CREATED, description = "Create a library", body = Library),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    ValidatedJson(payload): ValidatedJson<LibraryPayload>,
) -> ResponseResult<impl IntoResponse> {
    let result: Library = libraries::create(state.database_connection(), payload)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    get,
    path = Paths::LIBRARIES,
    tag = Tags::LIBRARIES,
    operation_id = "query_library",
    params(
        Pagination,
    ),
    responses(
        (status = OK, description = "Query libraries", body = Paginated<Library>),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn query(
    State(state): State<AppState>,
    _: AuthorizedClaims<UserAuthentication>,
    Query(pagination): Query<Pagination>,
) -> ResponseResult<impl IntoResponse> {
    let result = libraries::query(state.database_connection(), pagination)
        .await
        .map_err(CoreError::Database)?;

    let result: Paginated<Library> = result.into();

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = Paths::LIBRARIES_ID,
    tag = Tags::LIBRARIES,
    operation_id = "library_info",
    params(
        ("id" = Uuid, Path, description = "Library ID"),
    ),
    responses(
        (status = OK, description = "Get library information", body = Library),
        (status = NOT_FOUND, description = "Library not found", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn find(
    State(state): State<AppState>,
    _: AuthorizedClaims<UserAuthentication>,
    Path(id): Path<Uuid>,
) -> ResponseResult<impl IntoResponse> {
    let result: Library = libraries::get_by_id(state.database_connection(), id)
        .await?
        .into();

    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = Paths::LIBRARIES_ID,
    tag = Tags::LIBRARIES,
    operation_id = "update_library",
    params(
        ("id" = Uuid, Path, description = "Library ID"),
    ),
    request_body = LibraryPayload,
    responses(
        (status = OK, description = "Update library", body = Library),
        (status = NOT_FOUND, description = "Library not found", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn update(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<LibraryPayload>,
) -> ResponseResult<impl IntoResponse> {
    let result: Library = libraries::update(state.database_connection(), id, payload)
        .await?
        .into();

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = Paths::LIBRARIES_ID,
    tag = Tags::LIBRARIES,
    operation_id = "delete_library",
    params(
        ("id" = Uuid, Path, description = "Library ID"),
    ),
    responses(
        (status = OK, description = "Delete library"),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(id): Path<Uuid>,
) -> ResponseResult<impl IntoResponse> {
    libraries::delete(state.database_connection(), id)
        .await
        .map_err(CoreError::Database)?;

    Ok(Json(()))
}
