use axum::{http::StatusCode, Json};
use aide::axum::IntoApiResponse;
use serde_json::json;

pub(crate) async fn get_root() -> impl IntoApiResponse  {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}