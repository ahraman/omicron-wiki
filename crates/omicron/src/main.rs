use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::net::TcpListener;

use omicron::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenvy::dotenv()?;
    let server_url = std::env::var("SERVER_URL")?;

    let router = Router::new().route("/", get(root));

    let listener = TcpListener::bind(&server_url).await?;
    Ok(axum::serve(listener, router).await?)
}

async fn root() -> Result<Response, Error> {
    Ok("hello, world!".into_response())
}
