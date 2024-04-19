use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi, Url};

pub struct ApiDocsRouter;

impl ApiDocsRouter {
    const OPENAPI_JSON: &'static str = "/api-docs/openapi.json";
    const SWAGGER_UI: &'static str = "/api-docs/swagger-ui";
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Corgi API",
    ),
    paths(openapi),
    tags(
        (name = "OpenAPI"),
    ),
)]
struct ApiDocs;

#[utoipa::path(
    get,
    path = ApiDocsRouter::OPENAPI_JSON,
    tag = "OpenAPI",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDocs::openapi())
}

impl ApiDocsRouter {
    pub fn route() -> Router {
        let config = Config::new([Url::new("Corgi API", ApiDocsRouter::OPENAPI_JSON)]);
        let swagger = SwaggerUi::new(ApiDocsRouter::SWAGGER_UI).config(config);

        Router::new()
            .route(ApiDocsRouter::OPENAPI_JSON, get(openapi))
            .merge(swagger)
    }
}
