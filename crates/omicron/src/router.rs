use std::sync::Arc;

use axum::{
    Router, debug_handler,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::{App, AppState, Error};

pub fn build_router(app: Arc<App>) -> Router {
    let state = AppState(app);
    build_root_router().with_state(state)
}

fn build_root_router() -> Router<AppState> {
    Router::new().route("/", get(root))
}

#[debug_handler(state = AppState)]
async fn root(AppState(_): AppState) -> Result<Response, Error> {
    Ok("hello, world!".into_response())
}
