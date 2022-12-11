use crate::prelude::*;
use axum::{http::StatusCode, response::IntoResponse};
use std::fmt;
use tracing::instrument;

#[instrument]
pub(crate) async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NOT_FOUND_MESSAGE)
}

#[instrument]
pub(crate) fn internal_error<E: fmt::Display + fmt::Debug>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
