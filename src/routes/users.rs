use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::Error;
use crate::{config::ConfigState, database::{self, users::{remove_user, update_user}}, definitions::user::User};
use uuid::Uuid;
use aide::axum::IntoApiResponse;
use database::users::{find_user, create_user};

// Custom User struct for manual UUID validation
#[derive(Debug, Deserialize)]
pub struct NewUser {
    user_id: String,
    username: String,
}

#[axum::debug_handler]
pub async fn get_user(
    user_id_result: Result<Path<Uuid>, axum::extract::rejection::PathRejection>,
    State(config): State<Arc<ConfigState>>,
) -> impl IntoApiResponse {
    // Check if UUID is valid
    let user_id = match user_id_result {
        Ok(Path(id)) => id,
        Err(err) => {
            // Log the detailed error on the server
            eprintln!("Invalid UUID: {}", err);
            
            // Return error message to the client if UUID is invalid
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format" })),
            );
        }
    };

    // Proceed with finding the user if the UUID was valid
    match find_user(user_id, &config.pgpool).await {
        Ok(Some(user)) => (StatusCode::OK, Json(json!(user))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
        Err(err) => {
            eprintln!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
        }
    }
}

#[axum::debug_handler]
pub async fn post_user(State(config): State<Arc<ConfigState>>, Json(new_user): Json<NewUser>) -> impl IntoApiResponse {
    // Check if UUID is valid
    let user_id = match Uuid::parse_str(&new_user.user_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Invalid UUID: {err}");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "Invalid UUID format" })),
            );
        }
    };

    // Proceed to create the user
    let user = User {
        user_id,
        username: new_user.username.clone(),
    };

    let res = match create_user(user, &config.pgpool).await {
        Ok(Some(user)) => (StatusCode::CREATED, Json(json!(user))),
        Ok(None) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "User creation failed" })),
        ),
        Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            // Compare using `as_deref` to convert `Cow<'_, str>` to `Option<&str>`
            // 23505 is the SQL state for unique violation
            (
                StatusCode::CONFLICT,
                Json(json!({ "error": "User already exists" })),
            )
        },
        Err(err) => {
            eprintln!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error"})),
            )
        },
    };

    res
}

#[axum::debug_handler]
pub async fn put_user(State(config): State<Arc<ConfigState>>, Json(new_user): Json<NewUser>) -> impl IntoApiResponse {
    // Check if UUID is valid
    let user_id = match Uuid::parse_str(&new_user.user_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Invalid UUID: {err}");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "Invalid UUID format" })),
            );
        }
    };

    // Proceed to create the user
    let user = User {
        user_id,
        username: new_user.username.clone(),
    };

    let res = match update_user(user, &config.pgpool).await {
        Ok(Some(user)) => (StatusCode::CREATED, Json(json!(user))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
        Err(err) => {
            eprintln!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error"})),
            )
        },
    };

    res
}

#[axum::debug_handler]
pub async fn delete_user(user_id_result: Result<Path<Uuid>, axum::extract::rejection::PathRejection>, State(config): State<Arc<ConfigState>>) -> impl IntoApiResponse {
    // Check if UUID is valid
    let user_id = match user_id_result {
        Ok(Path(id)) => id,
        Err(err) => {
            // Log the detailed error on the server
            eprintln!("Invalid UUID: {}", err);
            
            // Return error message to the client if UUID is invalid
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format" })),
            );
        }
    };

    let res = match remove_user(user_id, &config.pgpool).await {
        Ok(Some(user)) => (StatusCode::CREATED, Json(json!(user))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
        Err(err) => {
            eprintln!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error"})),
            )
        },
    };

    res
}