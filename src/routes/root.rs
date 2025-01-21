use std::sync::Arc;
use axum::{extract::State, http::StatusCode, Json};
use aide::axum::IntoApiResponse;
use serde_json::json;
use crate::config::ConfigState;

pub(crate) async fn get_root(State(config): State<Arc<ConfigState>>) -> impl IntoApiResponse  {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}