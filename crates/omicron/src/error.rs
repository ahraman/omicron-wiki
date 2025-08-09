use std::path::PathBuf;

use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid asset name {0}")]
    AssetName(PathBuf),

    #[error(transparent)]
    Db(#[from] crate::db::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error(transparent)]
    Http(#[from] axum::http::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{self}").into_response()
    }
}
