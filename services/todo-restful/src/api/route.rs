use crate::{
    api::model::*,
    ioc::{ThreadSafeContainer, TodoFact},
    todo_service::TodoService,
};
use axum::Extension;
use axum::{extract::Path, Json};
use tracing::instrument;

#[instrument(skip(deps))]
#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Todo list.", body = [ResponseTodo]),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "todo",
)]
/// List all todos.
pub async fn list_todos<T: ThreadSafeContainer>(
    Extension(deps): Extension<T>,
) -> Json<Vec<ResponseTodo>> {
    let todos = deps.todo_factory().create();
    Json(
        todos
            .list_todos()
            .await
            .ok()
            .into_iter()
            .flat_map(|x| x.into_iter())
            .map(ResponseTodo::from)
            .collect(),
    )
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = RequestTodo,
    responses(
        (status = 200, description = "Todo created", body = ResponseTodo),
        (status = 415, description = "Unknown media type", body = ApiError),
        (status = 422, description = "Invalid JSON", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "todo",
)]
#[instrument(skip(deps))]
/// Create a new todo.
pub async fn new_todo<T: ThreadSafeContainer>(
    Json(req): Json<RequestTodo>,
    Extension(deps): Extension<T>,
) -> Result<Json<ResponseTodo>, Json<ApiError>> {
    let todos = deps.todo_factory().create();
    let res = todos
        .add_todo(req.text)
        .await
        .map_err(ApiError::from)
        .map_err(Json)?;

    Ok(Json(res.into()))
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo deleted", body = ResponseTodo),
        (status = 404, description = "Todo not found", body = ApiError),
        (status = 415, description = "Unknown media type", body = ApiError),
        (status = 422, description = "Invalid JSON", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    params(
        ("id" = usize, path, description = "ID of the Todo to delete"),
    ),
    tag = "todo",
)]
#[instrument(skip(deps))]
/// Delete an existing todo.
pub async fn delete_todo<T: ThreadSafeContainer>(
    Path((id,)): Path<(usize,)>,
    Extension(deps): Extension<T>,
) -> Result<Json<ResponseTodo>, Json<ApiError>> {
    let todos = deps.todo_factory().create();
    let res = todos
        .delete_todo(id)
        .await
        .map_err(ApiError::from)
        .map_err(Json)?;

    Ok(Json(res.into()))
}

#[instrument(skip(_deps))]
pub async fn health<T: ThreadSafeContainer>(Extension(_deps): Extension<T>) -> &'static str {
    "don't worry, I'm healthy"
}
