pub mod configurations;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub struct Routers;

impl Routers {}

impl Routers {
    pub fn route() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().merge(configurations::Routers::route())
    }
}
