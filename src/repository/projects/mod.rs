mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct ProjectSearchRecord {
    pub project_id: String,
    pub name: String,
    pub score: f32,
}

pub trait ProjectSearchRepository {
    async fn create(&self, project_id: &str, name: &str, value: &str) -> Result<(), QueryError>;

    async fn delete(&self, project_id: &str, value: &str) -> Result<(), QueryError>;

    async fn query<'a>(&self, terms: &'a Vec<&str>) -> Result<Vec<ProjectSearchRecord>, QueryError>;
}

pub enum ProjectSearchRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ProjectSearchRepository for ProjectSearchRepositoryOption {
    async fn create(&self, project_id: &str, name: &str, value: &str) -> Result<(), QueryError> {
        match self {
            ProjectSearchRepositoryOption::Postgres(db) => db.create(project_id, name, value).await,
        }
    }

    async fn delete(&self, project_id: &str, value: &str) -> Result<(), QueryError> {
        match self {
            ProjectSearchRepositoryOption::Postgres(db) => db.delete(project_id, value).await,
        }
    }

    async fn query<'a>(&self, terms: &'a Vec<&str>) -> Result<Vec<ProjectSearchRecord>, QueryError> {
        match self {
            ProjectSearchRepositoryOption::Postgres(db) => db.query(terms).await,
        }
    }
}

impl Default for ProjectSearchRepositoryOption {
    fn default() -> Self {
        ProjectSearchRepositoryOption::Postgres(Default::default())
    }
}