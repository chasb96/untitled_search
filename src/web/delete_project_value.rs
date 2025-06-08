use axum::extract::Path;
use or_status_code::OrInternalServerError;
use axum::http::StatusCode;

use crate::axum::extractors::project_search_repository::ProjectSearchRepositoryExtractor;
use crate::repository::projects::ProjectSearchRepository;

pub async fn delete_project_value(
    project_search_repository: ProjectSearchRepositoryExtractor,
    Path((project_id, value)): Path<(String, String)>
) -> Result<StatusCode, StatusCode> {
    project_search_repository
        .delete(&project_id, &value)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}