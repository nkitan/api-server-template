use std::sync::Arc;

use crate::{config::ConfigState, routes::{auth::login_user, root::get_root, users::{delete_user, post_user, put_user}}};
use crate::routes::users::get_user;
use axum_prometheus::PrometheusMetricLayer;
use axum_keycloak_auth::{instance::{KeycloakAuthInstance, KeycloakConfig}, layer::KeycloakAuthLayer, PassthroughMode, Url};
use axum::{Extension, Json};
use aide::axum::{
    routing::get,
    ApiRouter, IntoApiResponse,
};

// Serve pre-serialzed JSON
async fn serve_api(Extension(api_json): Extension<Arc<String>>) -> impl IntoApiResponse {
    Json((*api_json).clone())
}

// OpenAPI endpoints
pub fn open_api_router(config: Arc<ConfigState>) -> ApiRouter {
    ApiRouter::new()
    .api_route("/api.json", get(serve_api))
    .with_state(config)
}

// Protected endpoints
pub fn metrics_router() -> (ApiRouter, PrometheusMetricLayer<'static>) {
    let (mut prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    prometheus_layer.enable_response_body_size();
    
    let router = ApiRouter::new()
    .api_route("/metrics", get(|| async move { metric_handle.render() }));

    (router, prometheus_layer)
}

// Protector router layer
pub fn protect(router:ApiRouter, instance: Arc<KeycloakAuthInstance>) -> ApiRouter {
    router.layer(
        KeycloakAuthLayer::<String>::builder()
            .instance(instance)
            .passthrough_mode(PassthroughMode::Block)
            .persist_raw_claims(false)
            .expected_audiences(vec![String::from("account")])
            .required_roles(vec![String::from("user")])
            .build(),
    )
}

// Protected endpoints
pub fn private_router(config: Arc<ConfigState>) -> ApiRouter {
    // Create keycloak auth integration instance
    let keycloak_auth_instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse("http://localhost:8080/").unwrap())
            .realm(String::from("api-template"))
            .build(),
    );

    let unprotected_router = ApiRouter::new()
    .api_route("/users/{id}", get(get_user).delete(delete_user))
    .api_route("/users/{id}", axum::routing::put(put_user).into())
    .api_route("/users", axum::routing::post(post_user).into())
    .with_state(config.clone());
    
    protect(unprotected_router, keycloak_auth_instance.into())
}

// Publically available endpoints
pub fn public_router(config: Arc<ConfigState>) -> ApiRouter {
    ApiRouter::new()
    .api_route("/", get(get_root))
    .api_route("/login", axum::routing::post(login_user).into())
    .with_state(config)
}