use axum::Json;
use axum::extract::Query;
use or_status_code::OrInternalServerError;
use serde::Serialize;
use serde::Deserialize;
use axum::http::StatusCode;
use futures::join;

use crate::axum::extractors::project_search_repository::ProjectSearchRepositoryExtractor;
use crate::axum::extractors::user_search_repository::UserSearchRepositoryExtractor;
use crate::repository::users::UserSearchRepository;
use crate::repository::users::UserSearchRecord;
use crate::repository::projects::ProjectSearchRepository;
use crate::repository::projects::ProjectSearchRecord;

#[derive(Deserialize)]
pub struct Search {
    #[serde(rename = "q")]
    pub query: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    #[serde(rename = "r")]
    pub result: Vec<SearchResultItem>,
}

#[derive(Serialize)]
pub enum SearchResultItem {
    #[serde(rename = "u")]
    User(UserResult),
    #[serde(rename = "p")]
    Project(ProjectResult),
} 

#[derive(Serialize)]
pub struct UserResult {
    #[serde(rename = "i")]
    pub user_id: String,
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "s")]
    pub score: f32,
}

#[derive(Serialize)]
pub struct ProjectResult {
    #[serde(rename = "i")]
    pub project_id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub score: f32,
}

impl From<UserSearchRecord> for SearchResultItem {
    fn from(record: UserSearchRecord) -> Self {
        SearchResultItem::User(UserResult {
            user_id: record.user_id,
            username: record.username,
            score: record.score,
        })
    }
}

impl From<ProjectSearchRecord> for SearchResultItem {
    fn from(record: ProjectSearchRecord) -> Self {
        SearchResultItem::Project(ProjectResult {
            project_id: record.project_id,
            name: record.name,
            score: record.score,
        })
    }
}

pub async fn query(
    user_search_repository: UserSearchRepositoryExtractor,
    project_search_repository: ProjectSearchRepositoryExtractor,
    Query(query): Query<Search>,
) -> Result<Json<SearchResult>, StatusCode> {
    let query: Vec<&str> = query.query
        .split(' ')
        .collect();

    let (results, projects_results) = join!(
        user_search_repository.query(&query),
        project_search_repository.query(&query),
    );

    let mut results: Vec<SearchResultItem> = results
        .or_internal_server_error()?
        .into_iter()
        .map(SearchResultItem::from)
        .collect();

    let mut projects_results = projects_results
        .or_internal_server_error()?
        .into_iter()
        .map(SearchResultItem::from)
        .collect();

    results.append(&mut projects_results);

    Ok(Json(SearchResult {
        result: results,
    }))
}