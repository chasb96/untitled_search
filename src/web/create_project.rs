use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;

use crate::axum::extractors::project_search_repository::ProjectSearchRepositoryExtractor;
use crate::repository::projects::ProjectSearchRepository;

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
    #[prost(string, tag = "3")]
    pub value: String,
}

pub async fn create_project(
    project_search_repository: ProjectSearchRepositoryExtractor,
    Protobuf(request): Protobuf<CreateProjectRequest>
) -> Result<StatusCode, StatusCode> {
    project_search_repository
        .create(&request.project_id, &request.project_name, &request.value)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::CREATED)
}