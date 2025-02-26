use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct SignUpRequest {
    #[garde(length(min = 3, max = 128))]
    #[schema(example = "root")]
    pub name: String,

    #[garde(ascii, length(min = 3, max = 64))]
    #[schema(example = "root")]
    pub username: String,

    #[garde(email)]
    #[schema(example = "root@corgi.media")]
    pub email: String,

    #[garde(length(min = 6, max = 128))]
    #[schema(example = "password")]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct PasswordSignInRequest {
    #[garde(ascii, length(min = 3, max = 64))]
    #[schema(example = "root")]
    pub account: String,

    #[garde(length(min = 6, max = 128))]
    #[schema(example = "password")]
    pub password: String,
}
