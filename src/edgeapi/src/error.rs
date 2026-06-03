// src/edge-api-rs/src/error.rs
use summer_web::ProblemDetails;

#[derive(Debug, thiserror::Error, ProblemDetails)]
pub enum ApiError {
    #[status_code(400)]
    #[title("Bad Request")]
    #[error("bad request: {0}")]
    BadRequest(String),

    #[status_code(404)]
    #[title("Not Found")]
    #[error("resource not found")]
    NotFound,

    #[status_code(503)]
    #[title("Backend Unavailable")]
    #[error("backend service unavailable")]
    BackendUnavailable,

    #[status_code(500)]
    #[title("Internal Server Error")]
    #[error("internal server error")]
    Internal,
}

impl From<tonic::Status> for ApiError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::NotFound => ApiError::NotFound,
            tonic::Code::InvalidArgument => ApiError::BadRequest(status.message().to_string()),
            tonic::Code::Unavailable | tonic::Code::DeadlineExceeded => {
                ApiError::BackendUnavailable
            }
            _ => ApiError::Internal,
        }
    }
}