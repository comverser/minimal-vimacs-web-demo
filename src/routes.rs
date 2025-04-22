use crate::app::controllers::home;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home::index))
        .route("/api/log", get(home::log))
        .nest_service("/assets", ServeDir::new("assets"))
}
