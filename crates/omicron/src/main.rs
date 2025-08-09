use std::{net::SocketAddr, sync::Arc};

use axum::{ServiceExt, extract::Request};
use tokio::net::TcpListener;

use omicron::{App, Config, Error, build_router};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenvy::dotenv()?;

    let config = Config::from_env()?;
    let app = Arc::new(App::new(config).await?);

    let router = ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(
        NormalizePathLayer::trim_trailing_slash().layer(build_router(app.clone())),
    );

    let listener = TcpListener::bind(&app.config.server_url).await?;
    println!("> listening on: {}", app.config.server_url);
    Ok(axum::serve(listener, router).await?)
}
