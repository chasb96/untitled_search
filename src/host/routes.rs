use axum::{routing::{get, post}, Router};

use super::web::{query, register};

pub trait SearchRouter {
    fn register_search_routes(self) -> Self;
}

impl SearchRouter for Router {
    fn register_search_routes(self) -> Self {
        self.route("/search", get(query))
            .route("/search/register", post(register))
    }
}