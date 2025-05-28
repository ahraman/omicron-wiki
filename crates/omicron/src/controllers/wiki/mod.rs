pub mod page;

use axum::{
    debug_handler,
    response::{IntoResponse, Redirect, Response},
};

use crate::{AppState, Error};

#[debug_handler(state = AppState)]
pub async fn root(AppState(_): AppState) -> Result<Response, Error> {
    Ok(Redirect::to("/w/page?title=main").into_response())
}
