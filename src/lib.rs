use ::axum::{routing::get, Router};
use routes::SearchRouter;

mod routes;
mod health;
mod configuration;
mod web;
mod repository;
mod axum;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_search_routes()
}