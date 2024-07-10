mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct UserSearchRecord {
    pub user_id: String,
    pub username: String,
    pub score: f32,
}

pub trait UserSearchRepository {
    async fn create(&self, user_id: &str, username: &str) -> Result<(), QueryError>;

    async fn query<'a>(&self, terms: &'a Vec<&str>) -> Result<Vec<UserSearchRecord>, QueryError>;
}

pub enum UserSearchRepositoryOption {
    Postgres(PostgresDatabase),
}

impl UserSearchRepository for UserSearchRepositoryOption {
    async fn create(&self, user_id: &str, username: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(user_id, username).await,
        }
    }

    async fn query<'a>(&self, terms: &'a Vec<&str>) -> Result<Vec<UserSearchRecord>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.query(terms).await,
        }
    }
}

impl Default for UserSearchRepositoryOption {
    fn default() -> Self {
        Self::Postgres(Default::default())
    }
}