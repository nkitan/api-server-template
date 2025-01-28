use axum::{
    body::Body, http::{header::HeaderValue, Request, Response}, middleware::Next
};
use crate::definitions::logging::get_included_paths;

// This is the middleware that adds headers to specific endpoints.
pub async fn ignore_logs(req: Request<Body>, next: Next) -> Response<Body> {
    // Extract the path from the request
    let path = req.uri().path().to_string();

    // Check if the path is in the list of included paths
    let included_paths = get_included_paths();
    
    // If the path is included, add a header to the response
    let mut response = next.run(req).await;
    if included_paths.contains(&path) {
        response.headers_mut().insert(
            "X-Ignore-Log",
            HeaderValue::from_static("true"),
        );
    }

    response
}
