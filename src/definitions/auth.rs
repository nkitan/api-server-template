use serde::Deserialize;

// Custom User struct for manual UUID validation
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u16,
	pub refresh_expires_in: u16,
	pub refresh_token: String, 
    pub token_type: String,
    pub id_token: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: u32,
    pub session_state: String,
    pub scope: String,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String, 
	pub refresh_expires_in: u16,
}