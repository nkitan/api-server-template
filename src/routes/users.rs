use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde_json::json;
use sqlx::{Pool, Postgres};
use crate::{config::ConfigState, definitions::user::User};
use uuid::Uuid;
use aide::axum::IntoApiResponse;

async fn find_user(user_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    let row= sqlx::query_as::<_, User>(
    r#"
        SELECT user_id, username
        FROM users
        WHERE user_id = $1
    "#,)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_user(Path(user_id): Path<Uuid>, State(config): State<Arc<ConfigState>>) -> impl IntoApiResponse {
    let res = match find_user(user_id, &config.pgpool).await {
        Ok(Some(user)) => (StatusCode::OK, Json(json!(user))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" }))
        ),
        Err(err) => {
            println!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error"}))
            )
        },
    };

    res
}