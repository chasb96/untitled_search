use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::repository::users::UserSearchRepositoryOption;

pub struct UserSearchRepositoryExtractor(UserSearchRepositoryOption);

impl Deref for UserSearchRepositoryExtractor {
    type Target = UserSearchRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UserSearchRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for UserSearchRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(UserSearchRepositoryExtractor::default())
    }
}