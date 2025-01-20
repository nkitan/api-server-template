use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::{config::ConfigState, definitions::user::User};
use uuid::Uuid;
use aide::axum::IntoApiResponse;

pub async fn get_user(Path(user_id): Path<Uuid>, State(config): State<Arc<ConfigState>>) -> impl IntoApiResponse {
    let user = User {
        user_id,
        username: "LOLWABOI".to_string()
    };

    (StatusCode::OK, Json(user))
}