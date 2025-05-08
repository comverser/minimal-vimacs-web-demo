mod app;
mod routes;

use app::state::AppState;
use shuttle_runtime::SecretStore;
use tower_livereload::LiveReloadLayer;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let openai_api_key = secrets
        .get("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not found");
    let auth_key = secrets.get("AUTH").expect("AUTH key not found");
    let state = AppState::new(openai_api_key, auth_key);

    let mut router = routes::create_router(state.clone());

    if cfg!(debug_assertions) {
        println!("Running in development mode with livereload enabled.");
        router = router.layer(LiveReloadLayer::new());
    }

    Ok(router.into())
}
