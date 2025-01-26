mod routes;
mod definitions;
mod config;
mod database;

#[macro_use]
mod custom;

use aide::{
    axum::{
        routing::get,
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum_keycloak_auth::{instance::{KeycloakAuthInstance, KeycloakConfig}, layer::KeycloakAuthLayer, PassthroughMode, Url};

use std::sync::Arc;
use anyhow::Result;
use axum::{Extension, Json};

use config::ConfigState;
use routes::{auth::login_user, root::get_root, users::{delete_user, post_user, put_user}};
use routes::users::get_user;
use axum_prometheus::PrometheusMetricLayer;

// Serve pre-serialzed JSON
async fn serve_api(Extension(api_json): Extension<Arc<String>>) -> impl IntoApiResponse {
    Json((*api_json).clone())
}

pub fn public_router(config: Arc<ConfigState>) -> ApiRouter {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    ApiRouter::new()
    .api_route("/", get(get_root))
    .api_route("/login", axum::routing::post(login_user).into())
    .api_route("/api.json", get(serve_api))
    .api_route("/metrics", get(|| async move { metric_handle.render() }))
    .layer(prometheus_layer)
    .with_state(config)
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

    // Create keycloak auth integration instance
    let keycloak_auth_instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse("http://localhost:8080/").unwrap())
            .realm(String::from("api-template"))
            .build(),
    );

    let app = ApiRouter::new()
    .api_route("/users/{id}", get(get_user).delete(delete_user))
    .api_route("/users", axum::routing::post(post_user).into())
    .api_route("/users", axum::routing::put(put_user).into())
    .with_state(config.clone())
    .layer(
        KeycloakAuthLayer::<String>::builder()
            .instance(keycloak_auth_instance)
            .passthrough_mode(PassthroughMode::Block)
            .persist_raw_claims(false)
            .expected_audiences(vec![String::from("account")])
            .required_roles(vec![String::from("administrator")])
            .build(),
    )
    // Merge public routes
    .merge(public_router(config))

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