use axum::{extract::Query, Json};
use log::error;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;

use crate::host::axum::extractors::search_repository::SearchRepositoryExtractor;
use crate::host::repository::search::SearchRepository;

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
pub struct SearchResultItem {
    #[serde(rename = "t")]
    pub result_type: i16,
    #[serde(rename = "v")]
    pub value: String,
}

pub async fn query(
    search_repository: SearchRepositoryExtractor,
    Query(query): Query<Search>,
) -> Result<Json<SearchResult>, StatusCode> {
    let results = search_repository
        .search(&query.query)
        .await
        .map_err(|e| {
            error!("{:?}", e); 
            
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(SearchResult {
        result: results
            .into_iter()
            .map(|(result_type, value)| SearchResultItem {
                result_type,
                value,
            })
            .collect()
    }))
}