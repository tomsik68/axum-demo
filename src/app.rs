use crate::ioc::ThreadSafeContainer;
use axum::{routing::get, Extension, Router};

pub fn create_app_with_spec<T: ThreadSafeContainer>(c: T) -> Router {
    use crate::endpoint::*;

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root::<T>))
        .route("/health", get(health::<T>))
        .layer(Extension(c))
}
