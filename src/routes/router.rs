use axum::{routing::MethodRouter, Router};

pub fn get_router() {
    Router::new()
}

pub fn build_route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
