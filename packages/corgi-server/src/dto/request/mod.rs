mod account;

use std::marker::PhantomData;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Request},
    http::request::Parts,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use corgi_core::{
    entities::user,
    utils::{authentication::Authentication, claims::Claims},
};
use garde::Validate;
use serde::de::DeserializeOwned;

use crate::state::AppState;
pub use account::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate<Context = ()>,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = super::ErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

pub struct AuthorizedClaims<T> {
    pub user: user::Model,
    _marker: PhantomData<T>,
}

impl<T> AuthorizedClaims<T>
where
    T: Authentication,
{
    pub fn new(user: user::Model) -> Self {
        Self {
            user,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T> FromRequestParts<AppState> for AuthorizedClaims<T>
where
    T: Authentication,
{
    type Rejection = super::ErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let claims = Claims::decode(bearer.token(), &state.keyring().public_key)
            .map_err(corgi_core::error::Error::JWT)?;

        let user = T::authenticate(state.database_connection(), &claims).await?;

        Ok(AuthorizedClaims::new(user))
    }
}
