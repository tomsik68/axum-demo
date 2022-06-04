use std::future::Ready;
use std::sync::{Arc, Mutex};

use crate::todo_service::{Todo, TodoError, TodoService};

#[derive(Clone, Default)]
pub struct InMemoryTodo {
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl InMemoryTodo {
    pub fn new(todos: Arc<Mutex<Vec<Todo>>>) -> Self {
        Self { todos }
    }
}

impl TodoService for InMemoryTodo {
    type AddTodoFuture = Ready<Result<Todo, TodoError>>;
    fn add_todo(&self, text: String) -> Self::AddTodoFuture {
        let mut todos = self.todos.try_lock().unwrap();
        let id = todos.len();
        let added = Todo { id, text };
        todos.push(added.clone());
        std::future::ready(Ok(added))
    }

    type ListTodosFuture = Ready<Result<Vec<Todo>, TodoError>>;
    fn list_todos(&self) -> Self::ListTodosFuture {
        std::future::ready(Ok(self.todos.try_lock().unwrap().clone()))
    }

    type GetTodoFuture = Ready<Result<Todo, TodoError>>;
    fn get_todo(&self, id: usize) -> Self::GetTodoFuture {
        std::future::ready(
            self.todos
                .try_lock()
                .unwrap()
                .get(id)
                .cloned()
                .ok_or(TodoError::TodoNotFound(id)),
        )
    }

    type DeleteTodoFuture = Ready<Result<Todo, TodoError>>;
    fn delete_todo(&self, id: usize) -> Self::DeleteTodoFuture {
        let mut todos = self.todos.try_lock().unwrap();
        let resp = todos.get(id).cloned().ok_or(TodoError::TodoNotFound(id));
        if id < todos.len() {
            todos.remove(id);
        }
        std::future::ready(resp)
    }
}
