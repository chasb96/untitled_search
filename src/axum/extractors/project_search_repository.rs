use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::repository::projects::ProjectSearchRepositoryOption;

pub struct ProjectSearchRepositoryExtractor(ProjectSearchRepositoryOption);

impl Deref for ProjectSearchRepositoryExtractor {
    type Target = ProjectSearchRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ProjectSearchRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ProjectSearchRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(ProjectSearchRepositoryExtractor::default())
    }
}