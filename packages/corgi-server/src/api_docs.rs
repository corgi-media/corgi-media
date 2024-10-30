use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Corgi API",))]
pub struct ApiDocs;

pub struct ApiTags;

impl ApiTags {
    pub const SYSTEM: &'static str = "System";
    pub const CONFIGURATIONS: &'static str = "Configurations";
}
