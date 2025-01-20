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

// Wrap the `OpenApi` document in an `Arc` to avoid cloning on each request.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = Arc::new(ConfigState::from_env().await?);

    let app = ApiRouter::new()
    .api_route("/", get(get_root))
    .api_route("/users/{id}", get(get_user))
    .route("/api.json", get(serve_api))
    .with_state(config.clone());

    let mut api = OpenApi {
        info: Info {
            description: Some("API Server Template".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let bind_url = format!("{}:{}", config.env.hostname, config.env.port);
    let listener = tokio::net::TcpListener::bind(bind_url).await.unwrap();
    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();

    Ok(())
}