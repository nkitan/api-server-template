use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use schemars::JsonSchema;


// User Struct
#[derive(Serialize, Deserialize, JsonSchema, FromRow, Debug)]
pub(crate) struct User {
    pub(crate) user_id: Uuid,
    pub(crate) username: String,
    pub email: Option<String>,  // Update User struct to include email
}


// Custom User struct for manual UUID validation
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub(crate) user_id: String,
    pub(crate) username: Option<String>,
    pub(crate) email: Option<String>, // New optional email field
}
