use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use corgi_core::{auth::authentication::AdminAuthentication, filesystem, types::FileEntry};

use crate::{
    dto::{AuthorizedClaims, ResponseResult},
    openapi::Tags,
    routers::Paths,
};

#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct PathQuery {
    pub path: Option<String>,
}

#[utoipa::path(
  get,
  path = Paths::SYSTEM_FS_DIRECTORIES,
  tag = Tags::SYSTEM,
  operation_id = "list_directories",
  summary = "List directories",
  params(PathQuery),
  responses(
      (status = OK, body = Vec<FileEntry>)
  ),
  security(
    ("JWT" = [])
  )
)]
pub async fn directories(
    _: AuthorizedClaims<AdminAuthentication>,
    Query(query): Query<PathQuery>,
) -> ResponseResult<impl IntoResponse> {
    let result = filesystem::ls(query.path, true);

    Ok(Json(result))
}

#[utoipa::path(
  get,
  path = Paths::SYSTEM_FS_FILES,
  tag = Tags::SYSTEM,
  operation_id = "list_files",
  summary = "List files",
  params(PathQuery),
  responses(
      (status = OK, body = Vec<FileEntry>)
  ),
  security(
    ("JWT" = [])
  )
)]
pub async fn files(
    _: AuthorizedClaims<AdminAuthentication>,
    Query(query): Query<PathQuery>,
) -> ResponseResult<impl IntoResponse> {
    let result = filesystem::ls(query.path, false);

    Ok(Json(result))
}
