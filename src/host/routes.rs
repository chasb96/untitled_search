use axum::{routing::get, Router};

use super::web::query;

pub trait SearchRouter {
    fn register_search_routes(self) -> Self;
}

impl SearchRouter for Router {
    fn register_search_routes(self) -> Self {
        self.route("/search", get(query))
    }
}