use std::{collections::HashMap, sync::Arc};

use aide::axum::IntoApiResponse;
use axum::{extract::{Json, State}, http::StatusCode};
use serde_json::json;

use crate::{config::ConfigState, definitions::auth::{LoginResponse, LoginUser, TokenResponse}};

pub async fn login_user(
    State(config): State<Arc<ConfigState>>,
    Json(new_user): Json<LoginUser>
) -> impl IntoApiResponse {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("scope", "email openid");
    params.insert("client_id", &config.env.kc_client_id);
    params.insert("client_secret", &config.env.kc_client_secret);
    params.insert("username", &new_user.username);
    params.insert("password", &new_user.password);

    let resp = config.client.post(format!("{}{}",config.env.kc_server_addr.to_string(), config.env.kc_login_path.to_string()))
    .form(&params)
    .send()
    .await;

    let res = match resp {
        Ok(body) => {
            match body.json::<TokenResponse>().await {
                Ok(json_body) => {
                    // Create and return login_response
                    let login_response: LoginResponse = LoginResponse {
                        access_token: json_body.access_token,
                        token_type: json_body.token_type,
                        expires_in: json_body.expires_in,
                        refresh_token: json_body.refresh_token,
                        refresh_expires_in: json_body.refresh_expires_in,
                    };

                    (
                        StatusCode::OK,
                        Json(json!(login_response)),
                    )
                },
                Err(err) => {
                    eprintln!("Failed to parse TokenResponse: {err}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "status": "internal server error" })),
                    )
                },
            }
        },
        Err(err) => {
            eprintln!("Failed to log in: {err}");
            (
                StatusCode::FORBIDDEN,
                Json(json!({ "status": "invalid credentials" })),
            )
        },
    };
    res
}