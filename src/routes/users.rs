use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use serde_json::json;
use sqlx::Error;
use tracing::instrument;
use crate::{config::ConfigState, custom::validators::is_valid_email, database::{self, users::{remove_user, update_user}}, definitions::user::{NewUser, User}, expect_admin};
use uuid::Uuid;
use aide::axum::IntoApiResponse;
use database::users::{find_user, create_user};

#[instrument(skip(config))]
#[axum::debug_handler]
pub async fn get_user(
    Extension(token): Extension<KeycloakToken<String>>,
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

#[instrument(skip(config))]
#[axum::debug_handler]
pub async fn post_user(
    Extension(token): Extension<KeycloakToken<String>>,
    State(config): State<Arc<ConfigState>>,
    Json(new_user): Json<NewUser>
) -> impl IntoApiResponse {
    // Ensure user is admin
    expect_admin!(&token);

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
    
    // Unwrap `username` and validate it
    let username = match &new_user.username {
        Some(name) if !name.trim().is_empty() => name.clone(),
        _ => {
            eprintln!("Invalid username: must not be empty");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "Username must not be empty" })),
            );
        }
    };

    // Validate the email field if provided
    let email = match &new_user.email {
        Some(email) if is_valid_email(email) => Some(email.clone()),
        Some(_) => {
            eprintln!("Invalid email: does not match valid email format");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "Invalid email format" })),
            );
        }
        None => None, // No email provided, set to None
    };
    

    // Proceed to create the user
    let user = User {
        user_id,
        username,
        email,
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

#[instrument(skip(config))]
#[axum::debug_handler]
pub async fn put_user(
    Extension(token): Extension<KeycloakToken<String>>,
    user_id_result: Result<Path<Uuid>, axum::extract::rejection::PathRejection>,
    State(config): State<Arc<ConfigState>>,
    Json(new_user): Json<NewUser>,
) -> impl IntoApiResponse {
    // Ensure user is admin
    expect_admin!(&token);

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

    // Validate the email field if provided
    let _email = match &new_user.email {
        Some(email) if is_valid_email(email) => Some(email.clone()),
        Some(_) => {
            eprintln!("Invalid email: does not match valid email format");
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "Invalid email format" })),
            );
        }
        None => None, // No email provided, set to None
    };

    // Perform partial update
    let res = match update_user(
        user_id,
        new_user,
        &config.pgpool,
    )
    .await
    {
        Ok(Some(user)) => {
            (StatusCode::OK, Json(json!(user)))
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found or no fields to update" })),
        ),
        Err(err) => {
            eprintln!("Internal Server Error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error"})),
            )
        }
    };

    res
}

#[instrument(skip(config))]
#[axum::debug_handler]
pub async fn delete_user(
    Extension(token): Extension<KeycloakToken<String>>,
    user_id_result: Result<Path<Uuid>, axum::extract::rejection::PathRejection>,
    State(config): State<Arc<ConfigState>>
) -> impl IntoApiResponse {
    // Ensure user is admin
    expect_admin!(&token);

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
        Ok(Some(user)) => {
            println!("User {} deleted successfully", user.user_id);
            (StatusCode::ACCEPTED, Json(json!({"message": "User deleted successfully"})))
        },
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