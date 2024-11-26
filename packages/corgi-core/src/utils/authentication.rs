use corgi_database::{entities::user, orm::DatabaseConnection};

use super::claims::Claims;
use crate::{error::Error, services::user::find_by_id};

pub trait Authentication {
    fn authenticate(
        db: &DatabaseConnection,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<user::Model, Error>> + Send {
        async { find_by_id(db, claims.iss).await }
    }
}

pub struct UserAuthentication;

impl Authentication for UserAuthentication {}

pub struct AdminAuthentication;

impl Authentication for AdminAuthentication {}

pub struct MixedAuthentication;

impl Authentication for MixedAuthentication {}
