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
use tracing::{info_span, Span};

use std::{sync::Arc, time::Duration};
use anyhow::{Ok, Result};
use axum::{extract::MatchedPath, http::{Request, Response}, Extension};

use routers::{private_router, public_router, metrics_router, open_api_router};
use config::ConfigState;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() -> Result<()>{
    // Start tracing subscriber
    // Configure tracing with log rotation
    let file_appender = rolling::daily("logs", "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Configure logging
    let format_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_thread_ids(false) // Include thread IDs in logs
        .with_thread_names(false); // Include thread names in logs

    // Pick log level from RUST_LOG or use info
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });

    // Initialise log subscriber
    tracing_subscriber::registry()
        .with(format_layer)
        .with(filter_layer) // Set log level to DEBUG and above
        .init();

    // Create tracing layer
    
    let tracer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str)
                .unwrap_or("<unknown>")
                .to_string();

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path = %matched_path, // Store as a string in the span
            )
        })
        .on_request(move |request: &Request<_>, span: &Span| {
            // Skip logging requests to skip_paths
            let skip_paths = vec!["/metrics"];
            let path = request.uri().path();
            
            if skip_paths.contains(&path) {
                return;
            }
            
            tracing::debug!(parent: span, "received request: {:?}", request);
        })
        .on_response(move |response: &Response<_>, latency: Duration, span: &Span| {
            // Do not log responses marked as IGNORE_LOG = "true"
            match response.headers().get("IGNORE_LOG") {
                Some(value) => {
                    if value == "true"{
                        return;
                    }
                }
                None => {}
            }
            tracing::info!(parent: span, "response generated: {:?}, latency: {:?}", response, latency);
        });
    
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
    
    // Get Metrics Router
    let (metrics_router, prometheus_layer) = metrics_router();

    // Build App
    let app = ApiRouter::new()
    .merge(private_router(config.clone()))
    .merge(public_router(config.clone()))
    .merge(metrics_router)
    .layer(prometheus_layer)
    .merge(open_api_router(config.clone()))
    .layer(tracer)
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