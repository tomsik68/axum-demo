use crate::ioc::ThreadSafeContainer;
use axum::Extension;
use tracing::instrument;

#[instrument(skip(_deps))]
pub async fn root<T: ThreadSafeContainer>(Extension(_deps): Extension<T>) -> &'static str {
    tracing::info!("root");
    "Hello, World!"
}

#[instrument(skip(_deps))]
pub async fn health<T: ThreadSafeContainer>(Extension(_deps): Extension<T>) -> &'static str {
    "don't worry, I'm healthy"
}
