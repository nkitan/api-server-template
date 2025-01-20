use serde::{Deserialize, Serialize};
use uuid::Uuid;
use schemars::JsonSchema;


// User Struct
#[derive(Serialize, Deserialize, JsonSchema)]
pub(crate) struct User {
    pub(crate) user_id: Uuid,
    pub(crate) username: String,
}