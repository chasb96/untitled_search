use axum::{http::request, Json};
use axum_extra::protobuf::Protobuf;
use log::error;
use prost::{Enumeration, Message};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::host::{axum::extractors::search_repository::SearchRepositoryExtractor, repository::search::SearchRepository};

#[derive(Debug, PartialEq, Enumeration)]
pub enum ResultType {
    Project = 1,
    User = 2,
}

#[derive(Deserialize, Message)]
pub struct RegisterRequest {
    #[prost(enumeration = "ResultType", tag = "1")]
    pub result_type: i32,
    #[prost(string, tag = "2")]
    pub value: String,
}

pub async fn register(
    search_repository: SearchRepositoryExtractor,
    // Protobuf(request): Protobuf<RegisterRequest>,
    Json(request): Json<RegisterRequest>,
) -> StatusCode {
    let outcome = search_repository
        .create(request.result_type as i16, &request.value)
        .await;

    match outcome {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            error!("{:?}", e);
             
            StatusCode::INTERNAL_SERVER_ERROR
        },
    }
}