use axum::{extract::Query, Json};
use or_status_code::OrInternalServerError;
use projects::client::axum::extractors::ProjectsClient;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use users::client::axum::extractors::UsersClient;
use futures::join;

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
    pub user_id: i32,
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

impl From<users::client::SearchRecord> for SearchResultItem {
    fn from(record: users::client::SearchRecord) -> Self {
        SearchResultItem::User(UserResult {
            user_id: record.user_id,
            username: record.username,
            score: record.score,
        })
    }
}

impl From<projects::client::SearchRecord> for SearchResultItem {
    fn from(record: projects::client::SearchRecord) -> Self {
        SearchResultItem::Project(ProjectResult {
            project_id: record.project_id,
            name: record.name,
            score: record.score,
        })
    }
}

pub async fn query(
    users_client: UsersClient,
    projects_client: ProjectsClient,
    Query(query): Query<Search>,
) -> Result<Json<SearchResult>, StatusCode> {
    let (results, projects_results) = join!(
        search_users(users_client, &query.query),
        search_projects(projects_client, &query.query),
    );

    let mut results = results.or_internal_server_error()?;
    let mut projects_results = projects_results.or_internal_server_error()?;

    results.append(&mut projects_results);

    Ok(Json(SearchResult {
        result: results,
    }))
}

async fn search_users(users_client: UsersClient, query: &str) -> Result<Vec<SearchResultItem>, StatusCode> {
    Ok(
        users_client
            .search(&query)
            .await
            .or_internal_server_error()?
            .records
            .into_iter()
            .map(SearchResultItem::from)
            .collect()
    )
}

async fn search_projects(projects_client: ProjectsClient, query: &str) -> Result<Vec<SearchResultItem>, StatusCode> {
    Ok(
        projects_client
            .search(&query)
            .await
            .or_internal_server_error()?
            .records
            .into_iter()
            .map(SearchResultItem::from)
            .collect()
    )
}