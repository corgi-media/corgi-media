use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi, Url};

use crate::state::AppState;

pub struct Routers;

impl Routers {
    pub const OPENAPI_JSON: &'static str = "/api-docs/openapi.json";
    pub const SWAGGER_UI: &'static str = "/api-docs/swagger-ui";
}

impl Routers {
    pub fn route(api: OpenApi) -> Router<AppState> {
        let config = Config::new([Url::new("Corgi API", Routers::OPENAPI_JSON)]);
        let swagger = SwaggerUi::new(Routers::SWAGGER_UI)
            .url(Routers::OPENAPI_JSON, api)
            .config(config);

        Router::new().merge(swagger)
    }
}
