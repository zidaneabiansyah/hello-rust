mod db;
mod handlers;
mod models;

use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use db::Db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let db = Db::baru();

    let app = Router::new()
        .route("/api/items", get(handlers::list_items).post(handlers::create_item))
        .route(
            "/api/items/{id}",
            get(handlers::get_item)
                .patch(handlers::update_item)
                .delete(handlers::delete_item),
        )
        .layer(CorsLayer::permissive())
        .with_state(db);

    let addr = "0.0.0.0:3000";
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
