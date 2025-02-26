use serde::Serialize;
use utoipa::ToSchema;

use corgi_core::{
    chrono::{DateTime, NaiveDate, Utc},
    entities::user,
    uuid::Uuid,
};

#[derive(Serialize, ToSchema)]
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
