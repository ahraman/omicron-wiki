use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv().unwrap();
    let server_url = std::env::var("SERVER_URL").unwrap();

    let router = Router::new().route("/", get(root));

    let listener = TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> String {
    "hello, world!".to_string()
}
