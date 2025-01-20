use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User Struct
#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) user_id: Uuid,
    pub(crate) username: String,
}