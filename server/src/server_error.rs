use axum::response::IntoResponse;
use http::StatusCode;
use docsmith_base::error::DocsmithError;

// Make our own error that wraps `anyhow::Error`.
pub struct ServerError(pub DocsmithError);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl From<http::Error> for ServerError {
    fn from(error: http::Error) -> Self {
        Self(error.into())
    }
}