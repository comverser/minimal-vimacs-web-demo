use crate::app::{controllers::home, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{services::ServeDir, validate_request::ValidateRequestHeaderLayer};

pub fn create_router(state: AppState) -> Router {
    let auth_key = state.auth_key.clone();

    Router::new()
        .route("/api/log", get(home::get_log))
        .route("/api/frame", post(home::post_frame))
        .with_state(state)
        .route("/", get(home::get_index))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(ValidateRequestHeaderLayer::basic(&auth_key, &auth_key))
}
