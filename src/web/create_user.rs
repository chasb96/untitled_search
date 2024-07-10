use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;

use crate::axum::extractors::user_search_repository::UserSearchRepositoryExtractor;
use crate::repository::users::UserSearchRepository;

#[derive(Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub username: String,
}

pub async fn create_user(
    user_search_repository: UserSearchRepositoryExtractor,
    Protobuf(request): Protobuf<CreateUserRequest>
) -> Result<StatusCode, StatusCode> {
    user_search_repository
        .create(&request.user_id, &request.username)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::CREATED)
}