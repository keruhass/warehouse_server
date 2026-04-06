mod dto;
mod errors;
mod handlers;
mod routes;
mod state;

use std::env;
use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use dashmap::DashMap;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    dotenv::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL must be set in .env file or environment")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Couldn't connect to database")?;

    println!("Database connection success");

    let state = Arc::new(AppState {
        db: pool,
        supplier_cache: DashMap::new(),
    });

    let app = Router::new()
        .nest("/api", routes::router())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
