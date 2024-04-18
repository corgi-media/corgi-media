use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub fn docs_routers() -> Router {
    let config = Config::new(["/api-docs/openapi.json"]);
    let swagger = SwaggerUi::new("/api-docs/swagger-ui").config(config);

    Router::new()
        .route("/api-docs/openapi.json", get(openapi))
        .merge(swagger)
}
