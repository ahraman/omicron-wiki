use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::Query,
    response::{Html, IntoResponse, Redirect, Response},
};
use serde_json::json;

use crate::{AppState, Error};

#[debug_handler(state = AppState)]
pub async fn get(
    app: AppState,
    Query(mut query): Query<HashMap<String, String>>,
) -> Result<Response, Error> {
    Ok(if let Some(title) = query.remove("title") {
        view_page(app, title, query).await?
    } else {
        Redirect::to("/w/page?title=main").into_response()
    })
}

async fn view_page(
    AppState(app): AppState,
    title: String,
    _: HashMap<String, String>,
) -> Result<Response, Error> {
    Ok(Html::from(app.render.render(
        "base",
        json!({
            "site": {
                "title": "Omicron",
            },
            "page": {
                "title": title,
            }
        }),
    )?)
    .into_response())
}
