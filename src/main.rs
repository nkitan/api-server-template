mod routes;
mod definitions;
mod config;

use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};

use axum::{Extension, Json};
use config::ConfigState;
use routes::root::get_root;
use std::sync::Arc;
use routes::users::get_user;

async fn serve_api(Extension(api_json): Extension<Arc<String>>) -> impl IntoApiResponse {
    Json((*api_json).clone())
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = Arc::new(ConfigState::from_env().await?);

    let mut api = OpenApi {
        info: Info {
            description: Some("API Server Template".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let app = ApiRouter::new()
    .api_route("/", get(get_root))
    .api_route("/users/{id}", get(get_user))
    .with_state(config.clone())
    .route("/api.json", get(serve_api))
    .finish_api(&mut api);

    // Serialize the OpenApi document to a JSON string
    let api_json = serde_json::to_string(&api).expect("Failed to serialize OpenAPI document");
    let shared_api_json = Arc::new(api_json);


    let bind_url = format!("{}:{}", config.env.hostname, config.env.port);
    let listener = tokio::net::TcpListener::bind(bind_url).await.unwrap();
    axum::serve(
        listener,
        app
            // Expose the documentation to the handlers.
            .layer(Extension(shared_api_json))
            .into_make_service(),
    )
    .await
    .unwrap();

    Ok(())
}