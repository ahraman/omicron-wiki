use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{App, AppState, controllers::asset};

pub fn build_router(app: Arc<App>) -> Router {
    let state = AppState(app);
    build_root_router().with_state(state)
}

fn build_root_router() -> Router<AppState> {
    use crate::controllers::root;
    Router::new()
        .route("/", get(root))
        .route("/asset", get(asset::get))
        .nest("/w", build_wiki_router())
}

fn build_wiki_router() -> Router<AppState> {
    use crate::controllers::wiki::{page, root};
    Router::new()
        .route("/", get(root))
        .route("/page", get(page::get))
}
