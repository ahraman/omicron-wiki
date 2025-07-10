use std::collections::HashMap;

use axum::{
    body::Body,
    debug_handler,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{AppState, Error, asset::AssetKey};

#[debug_handler(state = AppState)]
pub async fn get(
    AppState(app): AppState,
    Query(mut query): Query<HashMap<String, String>>,
) -> Result<Response, Error> {
    let file = query.remove("file").unwrap();
    Ok(match app.asset_manager.load(AssetKey::new(file)) {
        Ok(asset) => Response::builder()
            .header("Content-Type", asset.meta.content_type.to_string())
            .body(Body::from(asset.data.to_vec()))?,
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    })
}
