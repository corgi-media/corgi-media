use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Token {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordSignInPayload {
    #[garde(ascii, length(min = 3, max = 64))]
    #[schema(example = "root")]
    pub account: String,

    #[garde(length(min = 6, max = 128))]
    #[schema(example = "password")]
    pub password: String,
}
