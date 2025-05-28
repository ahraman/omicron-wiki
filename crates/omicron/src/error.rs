use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{}", self).into_response()
    }
}
