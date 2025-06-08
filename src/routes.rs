use axum::{routing::{get, post}, Router};

use crate::web::{create_project, create_user, delete_project_value};

use super::web::query;

pub trait SearchRouter {
    fn register_search_routes(self) -> Self;
}

impl SearchRouter for Router {
    fn register_search_routes(self) -> Self {
        self.route("/search", get(query))
            .route("/user", post(create_user))
            .route("/projects", post(create_project))
            .route("/projects/:project_id/value/:value", post(delete_project_value))
    }
}