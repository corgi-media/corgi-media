pub mod configurations;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub struct Routers;

impl Routers {
    pub const PATH: &'static str = "/system";
}

impl Routers {
    pub fn route() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().nest(Routers::PATH, configurations::Routers::route())
    }
}
