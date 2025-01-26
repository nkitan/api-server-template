use sqlx::{Pool, Postgres};
use crate::definitions::user::{User, NewUser};
use uuid::Uuid;

pub(crate) async fn find_user(user_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    let row= sqlx::query_as::<_, User>(
    r#"
        SELECT user_id, username, email
        FROM users
        WHERE user_id = $1
    "#,)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(row)
}

pub async fn create_user(user: User, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    // Insert the user into the database
    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (user_id, username, email)
        VALUES ($1, $2, $3)
        RETURNING user_id, username, email
        "#,
    )
    .bind(user.user_id) // Bind the user_id
    .bind(user.username) // Bind the username
    .bind(user.email) // Bind the username
    .fetch_optional(pool) // Fetch the inserted row
    .await?;

    Ok(result) // Return the inserted user
}
pub async fn update_user(
    new_user: NewUser,
    pool: &Pool<Postgres>,
) -> Result<Option<User>, sqlx::Error> {
    // Check if UUID is valid
    let user_id = match Uuid::parse_str(&new_user.user_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Invalid UUID: {err}");
            return Ok(None);  // Return None if UUID is invalid
        }
    };

    // Start building the query
    let mut query = String::from("UPDATE users SET ");
    let mut params: Vec<String> = Vec::new();
    let mut bindings: Vec<String> = Vec::new();

    // Dynamically add fields to update based on the NewUser struct
    if let Some(username) = new_user.username {
        if !username.trim().is_empty() {
            params.push("username = $1".to_string());
            bindings.push(username);
        }
    }

    if let Some(email) = new_user.email {
        if !email.trim().is_empty() {
            params.push("email = $2".to_string());
            bindings.push(email);
        }
    }

    // If no fields to update, return None
    if params.is_empty() {
        return Ok(None);
    }

    // Complete the query
    query.push_str(&params.join(", "));
    query.push_str(" WHERE user_id = $3 RETURNING user_id, username, email");

    // Execute the query, binding the appropriate parameters
    let result = sqlx::query_as::<_, User>(&query)
        .bind(bindings.get(0).unwrap()) // Bind the first parameter (username)
        .bind(bindings.get(1).unwrap_or(&"".to_string())) // Bind second parameter (if exists)
        .bind(user_id) // Bind user_id
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub(crate) async fn remove_user(user_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    let row= sqlx::query_as::<_, User>(
    r#"
        DELETE FROM users
        WHERE user_id = $1
        RETURNING user_id, username, email
    "#,)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(row)
}
