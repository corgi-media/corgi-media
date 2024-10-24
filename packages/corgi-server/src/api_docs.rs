use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Corgi API",))]
pub struct ApiDocs;

pub struct ApiTags;

impl ApiTags {
    pub const CONFIGURATIONS: &'static str = "Configurations";
}
