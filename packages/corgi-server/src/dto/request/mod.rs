mod account;
mod users;

use std::marker::PhantomData;

use axum::{
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Request},
    http::request::Parts,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization as AuthorizationHeader},
    TypedHeader,
};
use garde::Validate;
use serde::de::DeserializeOwned;

use corgi_core::{
    auth::{authentication::Authentication, authorization::Authorization, jwt::Claims},
    entities::user,
    error::Error as CoreError,
};

use crate::state::AppState;
pub use account::*;
pub use users::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate<Context = ()>,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = crate::error::Error;

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

impl<T> FromRequestParts<AppState> for AuthorizedClaims<T>
where
    T: Authentication + Authorization,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(AuthorizationHeader(bearer)) = parts
            .extract::<TypedHeader<AuthorizationHeader<Bearer>>>()
            .await?;

        let claims =
            Claims::decode(bearer.token(), &state.keyring().public_key).map_err(CoreError::JWT)?;

        let user = T::authenticate(state.database_connection(), &claims).await?;

        T::authorize(&user).map_err(CoreError::Authorization)?;

        Ok(AuthorizedClaims::new(user))
    }
}
