use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use corgi_core::{
    auth::authentication::AdminAuthentication,
    libraries,
    types::{LibraryDirectory, LibraryDirectoryPayload},
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
    path = Paths::LIBRARIES_DIRECTORIES,
    tag = Tags::LIBRARIES,
    operation_id = "add_library_directory",
    summary = "Add directory to library",
    params(
        ("library_id" = Uuid, Path, description = "Library ID"),
    ),
    request_body = LibraryDirectoryPayload,
    responses(
        (status = CREATED, body = LibraryDirectory),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(library_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<LibraryDirectoryPayload>,
) -> ResponseResult<impl IntoResponse> {
    let result: LibraryDirectory =
        libraries::directories::create(state.database_connection(), library_id, payload.path)
            .await?
            .into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    get,
    path = Paths::LIBRARIES_DIRECTORIES,
    tag = Tags::LIBRARIES,
    operation_id = "list_library_directory",
    summary = "List library directories",
    params(
        ("library_id" = Uuid, Path, description = "Library ID"),
    ),
    responses(
        (status = OK,  body = Vec<LibraryDirectory>),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn list(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(library_id): Path<Uuid>,
) -> ResponseResult<impl IntoResponse> {
    let result =
        libraries::directories::find_all_by_library_id(state.database_connection(), library_id)
            .await?;

    let result: Vec<LibraryDirectory> = result.into_iter().map(LibraryDirectory::from).collect();

    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = Paths::LIBRARIES_DIRECTORIES_ID,
    tag = Tags::LIBRARIES,
    operation_id = "update_library_directory",
    summary = "Update library directory",
    params(
        ("library_id" = Uuid, Path, description = "Library ID"),
        ("id" = i32, Path, description = "Directory ID"),
    ),
    request_body = LibraryDirectoryPayload,
    responses(
        (status = CREATED, body = LibraryDirectory),
        (status = UNPROCESSABLE_ENTITY, description = "Validation failed", body = ErrorResponse),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn update(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(path): Path<(Uuid, i32)>,
    ValidatedJson(payload): ValidatedJson<LibraryDirectoryPayload>,
) -> ResponseResult<impl IntoResponse> {
    let (_, id) = path;
    let result: LibraryDirectory =
        libraries::directories::update(state.database_connection(), id, payload.path)
            .await?
            .into();

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = Paths::LIBRARIES_DIRECTORIES_ID,
    tag = Tags::LIBRARIES,
    operation_id = "delete_library_directory",
    summary = "Delete library directory",
    params(
        ("library_id" = Uuid, Path, description = "Library ID"),
        ("id" = i32, Path, description = "Directory ID"),
    ),
    responses(
        (status = OK, body = ()),
    ),
    security(
        ("JWT" = [])
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    _: AuthorizedClaims<AdminAuthentication>,
    Path(path): Path<(Uuid, i32)>,
) -> ResponseResult<impl IntoResponse> {
    let (_, id) = path;
    libraries::directories::delete(state.database_connection(), id).await?;

    Ok(Json(()))
}
