use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use schemars::JsonSchema;


// User Struct
#[derive(Serialize, Deserialize, JsonSchema, FromRow, Debug)]
pub(crate) struct User {
    pub(crate) user_id: Uuid,
    pub(crate) username: String,
}