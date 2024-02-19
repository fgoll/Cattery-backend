use axum::{http::StatusCode, response::IntoResponse};

use self::jwt::AuthError;

mod jwt;
pub mod user;

pub enum ApiError {
    Auth(AuthError),
    INTERNAL(anyhow::Error),
    
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::INTERNAL(err.into())
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
