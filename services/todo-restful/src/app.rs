use crate::ioc::ThreadSafeContainer;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Extension, Json, Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

pub fn create_app_with_spec<T: ThreadSafeContainer>(c: T) -> Router {
    use crate::api::{delete_todo, health, list_todos, new_todo, ApiDoc};

    let config = Arc::new(Config::from("/openapi.json"));

    Router::new()
        .route("/todos", get(list_todos::<T>).post(new_todo::<T>))
        .route("/todos/:id", delete(delete_todo::<T>))
        .route("/health", get(health::<T>))
        .route(
            "/openapi.json",
            get({
                let doc = ApiDoc::openapi();
                move || async { Json(doc) }
            }),
        )
        .route(
            "/swagger/*tail",
            get(serve_swagger_ui).layer(Extension(config)),
        )
        .layer(Extension(c))
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
