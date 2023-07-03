extern crate diesel;

mod build;
mod connection_pool;
mod handlers;
mod models;
mod routes;
mod schema;

use crate::connection_pool::{get_pool, run_migrations};
use crate::handlers::internal_error;
use crate::routes::router::get_router;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_diesel_async_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let pool = get_pool(&db_url);
    run_migrations(pool.get().await.map_err(internal_error));
    let app = get_router().with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    // ignore this until axum 0.7 releases
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
