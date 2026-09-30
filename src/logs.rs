use axum::{
    body::Body,
    middleware::Next,
    http::{Request, StatusCode},
    response::Response,
};


pub async fn  error_logging_middleware(req: Request<Body>, next: Next) -> Response {
    let result = next.run(req).await;

    if result.status().is_server_error() {
        tracing::error!(
            "Server error in endpoint: {} - {}",
            result.status(),
            result.status().canonical_reason().unwrap_or("Unknown error")
        );
    } else if result.status() == StatusCode::METHOD_NOT_ALLOWED {
        tracing::error!(
            "Method Not Allowed (405) in endpoint: {} - {}",
            result.status(),
            result.status().canonical_reason().unwrap_or("Unknown error")
        );
    }

    result
}
