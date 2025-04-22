mod app;
mod routes;

use tower_livereload::LiveReloadLayer;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let mut router = routes::create_router();

    if cfg!(debug_assertions) {
        println!("Running in development mode with livereload enabled.");
        router = router.layer(LiveReloadLayer::new());
    }

    Ok(router.into())
}
