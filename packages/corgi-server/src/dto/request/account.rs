use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AccountCreateRequest {
    pub name: String,
    pub username: String,
    pub password: String,
}
