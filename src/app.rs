use crate::ioc::ThreadSafeContainer;
use axum::{
    routing::{delete, get},
    Extension, Router,
};

pub fn create_app_with_spec<T: ThreadSafeContainer>(c: T) -> Router {
    use crate::endpoint::*;

    Router::new()
        // `GET /` goes to `root`
        .route("/todos", get(list_todos::<T>).post(new_todo::<T>))
        .route("/todos/:id", delete(delete_todo::<T>))
        .route("/health", get(health::<T>))
        .layer(Extension(c))
}
