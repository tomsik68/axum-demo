use crate::todo_service::{Todo, TodoError};
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Deserialize, Debug, Serialize, Dummy, PartialEq, Component)]
pub struct RequestTodo {
    /// The text of the todo object.
    pub text: String,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Component)]
pub struct ResponseTodo {
    /// The identifier of the todo object.
    pub id: usize,
    /// The text of the todo object.
    pub text: String,
}

#[derive(Serialize, Component)]
pub struct ApiError {}

impl From<TodoError> for ApiError {
    fn from(_: TodoError) -> Self {
        Self {}
    }
}
impl From<Todo> for ResponseTodo {
    fn from(td: Todo) -> Self {
        Self {
            id: td.id,
            text: td.text,
        }
    }
}
