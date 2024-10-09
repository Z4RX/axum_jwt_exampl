use axum::{
    async_trait,
    extract::{Extension, FromRequestParts},
	http::request::Parts,
};
use axum_extra::typed_header::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use sqlx::PgPool;

use crate::{
    error::{ApiError, Error},
    model::User,
    utils::jwt,
};

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| Error::from(err))?;

		use axum::RequestPartsExt;
        let Extension(pool) = parts.extract::<Extension::<PgPool>>()
            .await
            .map_err(|err| Error::from(err))?;
        let claims = jwt::verify(bearer.token())?;
        Ok(User::find_by_id(claims.sub, &pool).await?)
    }
}