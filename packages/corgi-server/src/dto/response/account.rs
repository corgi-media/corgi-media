use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct Token {
    pub access_token: String,
}
