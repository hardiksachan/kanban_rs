use crate::error::{Error, Result};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use ctx;

#[derive(Debug)]
pub struct Ctx(pub ctx::Ctx);

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Result<ctx::Ctx>>()
            .ok_or(Error::AuthFailedCtxNotInRequestExt)?
            .clone()
            .map(|c| Ctx(c))
    }
}

impl From<Ctx> for ctx::Ctx {
    fn from(value: Ctx) -> Self {
        value.0
    }
}
