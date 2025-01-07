use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi, Url};

use super::Paths;
use crate::state::AppState;

pub fn route(api: OpenApi) -> Router<AppState> {
    let config =
        Config::new([Url::new("Corgi API", Paths::OPENAPI_JSON)]).persist_authorization(true);

    let swagger = SwaggerUi::new(Paths::SWAGGER_UI)
        .url(Paths::OPENAPI_JSON, api)
        .config(config);

    Router::new().merge(swagger)
}
