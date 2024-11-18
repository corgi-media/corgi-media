use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Corgi API",))]
pub struct Docs;

pub struct Tags;

impl Tags {
    pub const SYSTEM: &'static str = "System";
    // pub const CONFIGURATIONS: &'static str = "Configurations";
    pub const ACCOUNT: &'static str = "Account";
}
