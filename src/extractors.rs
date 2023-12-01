use std::str::FromStr;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct PathVec<T: FromStr>(pub Vec<T>);

#[async_trait]
impl<S: Send + Sync, T: FromStr> FromRequestParts<S> for PathVec<T> {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let path = parts.uri.path();
        let components = path
            .split('/')
            .skip(1)
            .map(|s| s.parse())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        let result = Self(components);
        Ok(result)
    }
}
