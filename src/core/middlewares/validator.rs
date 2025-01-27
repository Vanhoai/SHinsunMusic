use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::core::http::failure::{Failure, HttpFailure};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPayload<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedPayload<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = HttpFailure;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| HttpFailure::new(Failure::BadRequest(err.to_string())))?;

        value
            .validate()
            .map_err(|err| HttpFailure::new(Failure::BadRequest(err.to_string())))?;
        Ok(ValidatedPayload(value))
    }
}
