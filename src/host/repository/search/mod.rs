mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait SearchRepository {
    async fn create(&self, result_type: i16, value: &str) -> Result<(), QueryError>;

    async fn search(&self, query: &str) -> Result<impl Iterator<Item = (i16, String)>, QueryError>;
}

pub enum SearchRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SearchRepository for SearchRepositoryOption {
    async fn create(&self, result_type: i16, value: &str) -> Result<(), QueryError> {
        match self {
            SearchRepositoryOption::Postgres(db) => db.create(result_type, value).await,
        }
    }
    
    async fn search(&self, query: &str) -> Result<impl Iterator<Item = (i16, String)>, QueryError> {
        match self {
            SearchRepositoryOption::Postgres(db) => db.search(query).await,
        }
    }
}

impl Default for SearchRepositoryOption {
    fn default() -> Self {
        SearchRepositoryOption::Postgres(PostgresDatabase::default())
    }
}