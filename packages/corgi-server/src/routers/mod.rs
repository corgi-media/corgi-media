mod api_docs;
mod paths;
mod system;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::openapi;
use crate::state::AppState;
pub use paths::Paths;

pub struct AppRouter;

impl AppRouter {
    pub fn route() -> Router<AppState> {
        let (router, api) = OpenApiRouter::with_openapi(openapi::Docs::openapi())
            .merge(system::route())
            .split_for_parts();

        router.merge(api_docs::route(api))
    }
}
