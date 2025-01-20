use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{config::ConfigState, definitions::user::User};
use uuid::Uuid;

pub async fn get_user(Path(user_id): Path<Uuid>, State(config): State<Arc<ConfigState>>) -> impl IntoResponse {
    let user = User {
        user_id,
        username: "LOLWABOI".to_string()
    };

    (StatusCode::OK, Json(user))
}