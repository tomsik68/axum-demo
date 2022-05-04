use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tracing::instrument;

use crate::todo_service::{Todo, TodoError, TodoRequest, TodoResponse};
use tower::Service;

#[derive(Clone, Default)]
pub struct InMemoryTodo {
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl InMemoryTodo {
    pub fn new(todos: Arc<Mutex<Vec<Todo>>>) -> Self {
        Self { todos }
    }
}

impl Service<TodoRequest> for InMemoryTodo {
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync + 'static>>;
    type Response = TodoResponse;
    type Error = TodoError;

    #[instrument(skip(self))]
    fn call(&mut self, req: TodoRequest) -> Self::Future {
        let todos = Arc::clone(&self.todos);
        Box::pin(async move {
            let mut todos = todos.try_lock().unwrap();
            match req {
                TodoRequest::AddTodo(s) => {
                    let id = todos.len();
                    let added = Todo { id, text: s };
                    todos.push(added.clone());
                    Ok(TodoResponse::TodoAdded(added))
                }
                TodoRequest::GetTodo(id) => todos
                    .get(id)
                    .cloned()
                    .map(TodoResponse::TodoFound)
                    .ok_or(TodoError::TodoNotFound(id)),
                TodoRequest::DeleteTodo(id) => {
                    let resp = todos
                        .get(id)
                        .cloned()
                        .map(TodoResponse::TodoDeleted)
                        .ok_or(TodoError::TodoNotFound(id));
                    todos.remove(id);
                    resp
                }
                TodoRequest::ListTodos => Ok(TodoResponse::TodoList(todos.clone())),
            }
        })
    }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryTodo;

    fn assert_send_sync<T: Send + Sync + 'static>() {}

    #[test]
    fn test_in_memory_todo_send_sync() {
        assert_send_sync::<InMemoryTodo>();
    }
}
