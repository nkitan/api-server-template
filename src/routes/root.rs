use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use crate::config::ConfigState;

pub(crate) async fn get_root(State(config): State<Arc<ConfigState>>) -> impl IntoResponse {
    println!("{}", &config.env.hostname);
    (StatusCode::OK, "Hello, World!")
}