use chrono::{DateTime, NaiveDate, Utc};
use corgi_database::entities::user;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Serialize, ToSchema, PartialEq)]
pub enum UserIdentity {
    Administrator = 0,

    #[default]
    Normal = 1,
}

impl From<i32> for UserIdentity {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Administrator,
            _ => Self::Normal,
        }
    }
}

impl From<UserIdentity> for i32 {
    fn from(value: UserIdentity) -> Self {
        value as i32
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub identity: i32,
    pub birthday: Option<NaiveDate>,
    pub locked_until: Option<DateTime<Utc>>,
    pub disabled: bool,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<user::Model> for User {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            identity: value.identity,
            birthday: value.birthday,
            locked_until: value.locked_until,
            disabled: value.disabled,
            disabled_at: value.disabled_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateUserPayload {
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
