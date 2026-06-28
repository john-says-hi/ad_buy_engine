use ad_buy_engine_domain::{ApiErrorBody, ApiErrorCode, FieldError};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("{message}")]
    Api {
        status: StatusCode,
        code: ApiErrorCode,
        message: String,
        details: Vec<FieldError>,
    },
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("password error: {0}")]
    Password(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ServerError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::api(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::BadRequest,
            message,
            Vec::new(),
        )
    }

    pub fn validation(message: impl Into<String>, details: Vec<FieldError>) -> Self {
        Self::api(
            StatusCode::UNPROCESSABLE_ENTITY,
            ApiErrorCode::Validation,
            message,
            details,
        )
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::api(
            StatusCode::UNAUTHORIZED,
            ApiErrorCode::Unauthorized,
            message,
            Vec::new(),
        )
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::api(
            StatusCode::FORBIDDEN,
            ApiErrorCode::Forbidden,
            message,
            Vec::new(),
        )
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::api(
            StatusCode::NOT_FOUND,
            ApiErrorCode::NotFound,
            message,
            Vec::new(),
        )
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::api(
            StatusCode::CONFLICT,
            ApiErrorCode::Conflict,
            message,
            Vec::new(),
        )
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    fn api(
        status: StatusCode,
        code: ApiErrorCode,
        message: impl Into<String>,
        details: Vec<FieldError>,
    ) -> Self {
        Self::Api {
            status,
            code,
            message: message.into(),
            details,
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            Self::Api {
                status,
                code,
                message,
                details,
            } => (
                status,
                Json(ApiErrorBody {
                    code,
                    message,
                    details,
                }),
            )
                .into_response(),
            Self::Database(error) => {
                tracing::error!(error = %error, "database error");
                internal_response("Database request failed")
            }
            Self::Io(error) => {
                tracing::error!(error = %error, "io error");
                internal_response("Server file operation failed")
            }
            Self::Json(error) => {
                tracing::error!(error = %error, "json error");
                internal_response("Server data encoding failed")
            }
            Self::Password(error) => {
                tracing::error!(error = %error, "password error");
                internal_response("Password operation failed")
            }
            Self::Internal(error) => {
                tracing::error!(error = %error, "internal error");
                internal_response("Internal server error")
            }
        }
    }
}

fn internal_response(message: &'static str) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiErrorBody {
            code: ApiErrorCode::Internal,
            message: message.to_string(),
            details: Vec::new(),
        }),
    )
        .into_response()
}
