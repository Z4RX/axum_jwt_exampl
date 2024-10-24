use std::net::SocketAddr;
use clap::Parser;
use tracing_subscriber::EnvFilter;
use axum_jwt_example::{config, app};

#[tokio::main]
async fn main() {
    use config::db::DbPool;

    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let pg_pool = sqlx::PgPool::retrieve().await;
    let config = config::env::ServerConfig::parse();
    let addr = SocketAddr::from((config.host, config.port));
    tracing::debug!("listening on {}", addr);
    
    let app = app(pg_pool);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("server error: {:?}", err);
    }
}