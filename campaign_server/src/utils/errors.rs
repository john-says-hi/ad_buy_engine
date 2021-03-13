use actix_web::client::SendRequestError;
use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use uuid::parser::ParseError;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    #[display(fmt = "CA: Not connected")]
    NotConnected,
    #[display(fmt = "CA: Disconnected")]
    Disconnected,
    AgentDisconnected,
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
    ClientSendRequestError(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            }
            ApiError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ApiErrors
impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our apps grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError("Unknown database error".into())
            }
            diesel::result::Error::NotFound => ApiError::NotFound("not found".to_string()),
            diesel::result::Error::SerializationError(e) => {
                ApiError::NotFound("serialization error".to_string())
            }
            diesel::result::Error::RollbackTransaction => {
                ApiError::NotFound("rollback trx".to_string())
            }
            diesel::result::Error::QueryBuilderError(e) => {
                ApiError::NotFound("query builder err".to_string())
            }
            diesel::result::Error::InvalidCString(e) => {
                ApiError::NotFound("invalid c str".to_string())
            }
            diesel::result::Error::DeserializationError(e) => {
                ApiError::NotFound("deser err".to_string())
            }
            diesel::result::Error::AlreadyInTransaction => {
                ApiError::NotFound("already in trx".to_string())
            }
            diesel::result::Error::QueryBuilderError(e) => {
                ApiError::NotFound("query builder err".into())
            }
            _ => ApiError::NotFound("Diesel Error Exhausted".to_string()),
        }
    }
}

/// Convert PoolErrors to ApiErrors
impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}

/// Convert ParseErrors to ApiErrors
impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        ApiError::ParseError(error.to_string())
    }
}

/// Convert Thread BlockingErrors to ApiErrors
impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError("Thread blocking error".into()),
        }
    }
}

impl From<SendRequestError> for ApiError {
    fn from(error: SendRequestError) -> ApiError {
        ApiError::ClientSendRequestError(format!("{:?}", error))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err:std::io::Error)->ApiError{
        match err.kind() {
            _=>ApiError::InternalServerError(err.to_string())
        }
    }
}

impl From<BlockingError<diesel::result::Error>> for ApiError {
    fn from(err:BlockingError<diesel::result::Error>)->ApiError{
        ApiError::BlockingError("diesel blocking err detected".to_string())
    }
}
