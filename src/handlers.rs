use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, debug_handler,
};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    dto::{LoginInput, RegisterInput, TokenPayload},
    error::{ApiResult, Error},
    graphql::AppSchema,
    model::User,
    service::AuthService,
    utils::{jwt, validate_payload},
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

#[debug_handler]
pub async fn login(
	Extension(pool): Extension<PgPool>,
    Json(input): Json<LoginInput>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;
    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

#[debug_handler]
pub async fn register(
	Extension(pool): Extension<PgPool>,
    Json(input): Json<RegisterInput>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;
    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id)?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[debug_handler]
pub async fn graphql(
    schema: Extension<AppSchema>,
    user: Option<User>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(user)).await.into()
}