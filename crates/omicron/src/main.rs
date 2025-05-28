use std::sync::Arc;

use tokio::net::TcpListener;

use omicron::{App, Config, Error, build_router};

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenvy::dotenv()?;

    let config = Config::from_env()?;
    let app = Arc::new(App::new(config)?);
    let router = build_router(app.clone());

    let listener = TcpListener::bind(&app.config.server_url).await?;
    println!("> listening on: {}", app.config.server_url);
    Ok(axum::serve(listener, router).await?)
}
