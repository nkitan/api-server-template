mod routes;
mod definitions;
mod config;
mod database;
mod routers;

#[macro_use]
mod custom;

use aide::{
    axum::ApiRouter,
    openapi::{Info, OpenApi},
};

use std::sync::Arc;
use anyhow::Result;
use axum::Extension;

use routers::{private_router, public_router, metrics_router, open_api_router};
use config::ConfigState;

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
    .merge(private_router(config.clone()))
    .merge(public_router(config.clone()))
    .merge(open_api_router(config.clone()))
    .merge(metrics_router())

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