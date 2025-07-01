use axum::{
    routing::get,
    Router,
    Json,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// mod config;
mod db;
mod handlers;
mod models;
// mod collectors;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    // load environment variables
    dotenv::dotenv().ok();

    // set up logging
    tracing_subscriber::fmt::init();

    // connnect to the database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_pool = db::create_pool(&database_url).await?;

    // run migrations
    db::run_migrations(&db_pool).await?;

    // setting up the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/repositories", get(handlers::list_repositories))
        .layer(CorsLayer::permissive())
        .with_state(db_pool);
    
    // run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}