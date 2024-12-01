use corgi_core::chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[garde(length(min = 3, max = 128))]
    pub name: String,

    #[garde(ascii, length(min = 3, max = 64))]
    pub username: String,

    #[garde(length(min = 6, max = 128))]
    pub password: String,

    #[garde(range(min = 0, max = 1))]
    pub identity: i32,

    #[garde(skip)]
    pub birthday: Option<NaiveDate>,
}
