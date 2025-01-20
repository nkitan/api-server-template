mod routes;
mod definitions;
mod config;

use axum::{
    routing::get,
    Router,
};

use config::ConfigState;
use routes::root::get_root;
use std::sync::Arc;
use routes::users::get_user;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = Arc::new(ConfigState::from_env().await?);

    let app = Router::new()
    .route("/", get(get_root))
    .route("/users/{id}", get(get_user))
    .with_state(config.clone());

    let bind_url = format!("{}:{}", config.env.hostname, config.env.port);
    let listener = tokio::net::TcpListener::bind(bind_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}