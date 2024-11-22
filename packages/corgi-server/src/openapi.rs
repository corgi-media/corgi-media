use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Corgi API"),
    modifiers(&SecurityAddon),
)]
pub struct Docs;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "JWT",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}

pub struct Tags;

impl Tags {
    pub const SYSTEM: &'static str = "System";
    // pub const CONFIGURATIONS: &'static str = "Configurations";
    pub const ACCOUNT: &'static str = "Account";
}
