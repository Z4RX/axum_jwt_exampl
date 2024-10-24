#[macro_use]
extern crate async_graphql;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    routing::{get, post},
    Extension, Router
};

use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod dto;
mod error;
mod extractors;
mod graphql;
mod handlers;
mod model;
mod service;
mod sql;
mod utils;

pub mod config;

pub fn app(pg_pool: PgPool) -> Router {
    use self::graphql::{AppSchema, MutationRoot, QueryRoot};

    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool.to_owned())
        .finish();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(Extension(schema))
        .layer(Extension(pg_pool))
        .into_inner();

    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
        .route("/authorize", get(handlers::authorize))
        .route(
            "/graphql",
            get(handlers::graphql_playground).post(handlers::graphql),
        )
        .layer(middleware_stack)
}
