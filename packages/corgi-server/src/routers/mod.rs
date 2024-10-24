mod openapi;
mod system;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::api_docs::ApiDocs;
use crate::state::AppState;

pub struct AppRouter;

impl AppRouter {
    pub fn route() -> Router<AppState> {
        let (router, api) = OpenApiRouter::with_openapi(ApiDocs::openapi())
            .merge(system::Routers::route())
            .split_for_parts();

        router.merge(openapi::Routers::route(api))
    }
}
