use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};

use crate::{AppState, Error};

#[debug_handler(state = AppState)]
pub async fn get(
    AppState(_): AppState,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, Error> {
    Ok(if let Some(title) = query.get("title") {
        format!("you are at page `{title}`").into_response()
    } else {
        Redirect::to("/w/page?title=main").into_response()
    })
}
