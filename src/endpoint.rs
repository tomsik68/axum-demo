use crate::{
    ioc::{ThreadSafeContainer, TodoFact},
    todo_service::{Todo, TodoError, TodoRequest, TodoResponse},
};
use axum::Extension;
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use tower::Service;
use tracing::instrument;

#[derive(Serialize)]
pub struct ResponseTodo {
    pub id: usize,
    pub text: String,
}

#[derive(Serialize)]
pub struct ApiError;

impl From<TodoError> for ApiError {
    fn from(_: TodoError) -> Self {
        Self {}
    }
}

#[derive(Deserialize)]
pub struct RequestTodo {
    pub text: String,
}

impl From<Todo> for ResponseTodo {
    fn from(td: Todo) -> Self {
        Self {
            id: td.id,
            text: td.text,
        }
    }
}

#[instrument(skip(deps))]
pub async fn list_todos<T: ThreadSafeContainer>(
    Extension(deps): Extension<T>,
) -> Json<Vec<ResponseTodo>> {
    let mut todo_factory = deps.todo_factory().clone();
    let mut todos = todo_factory.create();
    Json(
        todos
            .call(TodoRequest::ListTodos)
            .await
            .ok()
            .iter()
            .flat_map(|resp| match resp {
                TodoResponse::TodoList(ls) => ls.into_iter(),
                _ => [].iter(),
            })
            .cloned()
            .map(ResponseTodo::from)
            .collect(),
    )
}

pub async fn new_todo<T: ThreadSafeContainer>(
    Json(req): Json<RequestTodo>,
    Extension(deps): Extension<T>,
) -> Result<Json<ResponseTodo>, Json<ApiError>> {
    let mut todos = deps.todo_factory().create();
    let res = todos
        .call(TodoRequest::AddTodo(req.text))
        .await
        .map_err(ApiError::from)
        .map_err(Json)?;

    match res {
        TodoResponse::TodoAdded(td) => Ok(Json(td.into())),
        _ => Err(Json(ApiError {})),
    }
}

pub async fn delete_todo<T: ThreadSafeContainer>(
    Path((id,)): Path<(usize,)>,
    Extension(deps): Extension<T>,
) -> Result<Json<ResponseTodo>, Json<ApiError>> {
    let mut todos = deps.todo_factory().create();
    let res = todos
        .call(TodoRequest::DeleteTodo(id))
        .await
        .map_err(ApiError::from)
        .map_err(Json)?;

    match res {
        TodoResponse::TodoDeleted(td) => Ok(Json(td.into())),
        _ => Err(Json(ApiError {})),
    }
}

#[instrument(skip(_deps))]
pub async fn health<T: ThreadSafeContainer>(Extension(_deps): Extension<T>) -> &'static str {
    "don't worry, I'm healthy"
}
