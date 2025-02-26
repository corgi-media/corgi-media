use corgi_core::chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[garde(length(min = 3, max = 128))]
    #[schema(example = "example")]
    pub name: String,

    #[garde(ascii, length(min = 3, max = 64))]
    #[schema(example = "example")]
    pub username: String,

    #[garde(email)]
    #[schema(example = "example@example.com")]
    pub email: String,

    #[garde(length(min = 6, max = 128))]
    #[schema(example = "password")]
    pub password: String,

    #[garde(range(min = 0, max = 1))]
    #[schema(example = 1)]
    pub identity: i32,

    #[garde(skip)]
    #[schema(example = "1998-08-10")]
    pub birthday: Option<NaiveDate>,
}
