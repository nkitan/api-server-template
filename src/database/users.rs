use sqlx::{Pool, Postgres};
use crate::definitions::user::User;
use uuid::Uuid;

pub(crate) async fn find_user(user_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
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

pub async fn create_user(user: User, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    // Insert the user into the database
    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (user_id, username)
        VALUES ($1, $2)
        RETURNING user_id, username
        "#,
    )
    .bind(user.user_id) // Bind the user_id
    .bind(user.username) // Bind the username
    .fetch_optional(pool) // Fetch the inserted row
    .await?;

    Ok(result) // Return the inserted user
}