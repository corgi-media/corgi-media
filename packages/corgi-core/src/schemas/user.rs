use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use corgi_database::entities::user;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub identity: UserIdentity,
    pub birthday: Option<NaiveDate>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
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
            identity: value.identity.into(),
            birthday: value.birthday,
            last_login_at: value.last_login_at,
            last_activity_at: value.last_activity_at,
            locked_until: value.locked_until,
            disabled: value.disabled,
            disabled_at: value.disabled_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub enum UserIdentity {
    Administrator,

    #[default]
    Normal,
}

impl From<i32> for UserIdentity {
    fn from(value: i32) -> Self {
        match value {
            0 => UserIdentity::Administrator,
            _ => UserIdentity::Normal,
        }
    }
}

impl From<UserIdentity> for i32 {
    fn from(value: UserIdentity) -> Self {
        match value {
            UserIdentity::Administrator => 0,
            UserIdentity::Normal => 1,
        }
    }
}
