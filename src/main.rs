mod routes;
mod definitions;
mod config;
mod database;

use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};

use std::sync::Arc;
use anyhow::Result;
use axum::{Extension, Json};

use config::ConfigState;
use routes::{root::get_root, users::post_user};
use routes::users::get_user;

// Serve pre-serialzed JSON
async fn serve_api(Extension(api_json): Extension<Arc<String>>) -> impl IntoApiResponse {
    Json((*api_json).clone())
}

#[tokio::main]
async fn main() -> Result<()>{

    // Load Configuration
    let config = Arc::new(ConfigState::from_env().await?);
    let app_name_string: String = format!("{}:{}", config.appname.as_str(), config.version.as_str());
    let bind_url = format!("{}:{}", config.env.hostname, config.env.port);

    // Describe OpenAPI handler
    let mut api = OpenApi {
        info: Info {
            description: Some(app_name_string),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let app = ApiRouter::new()
    .api_route("/", get(get_root))
    .api_route("/users/{id}", get(get_user))
    .api_route("/users", axum::routing::post(post_user).into())
    .with_state(config)
    // Routes mentioned under this do not require config access
    .route("/api.json", get(serve_api))
    // Create API Spec from routes defined before this
    .finish_api(&mut api);

    // Serialize the OpenAPI document to a JSON string for performance, store it in an atomic type for shared use
    let api_json = Arc::new(serde_json::to_string(&api).expect("Failed to serialize OpenAPI document"));

    // Start webserver on bind_url
    let listener = tokio::net::TcpListener::bind(bind_url).await.unwrap();

    // Serve axum routes as service with OpenAPI JSON as a layer
    axum::serve(
        listener,
        app
            // Expose the documentation to the handlers.
            .layer(Extension(api_json))
            .into_make_service(),
    )
    .await
    .unwrap();

    // Return empty result on exit
    Ok(())
}