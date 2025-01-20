use std::sync::Arc;
use axum::{extract::State, http::StatusCode};
use aide::axum::IntoApiResponse;
use crate::config::ConfigState;

pub(crate) async fn get_root(State(config): State<Arc<ConfigState>>) -> impl IntoApiResponse  {
    println!("{}", &config.env.hostname);
    (StatusCode::OK, "Hello, World!")
}