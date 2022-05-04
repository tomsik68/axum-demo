use tower::Service;
use trait_set::trait_set;

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct Todo {
    pub id: usize,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TodoRequest {
    AddTodo(String),
    ListTodos,
    DeleteTodo(usize),
    GetTodo(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TodoResponse {
    TodoAdded(Todo),
    TodoList(Vec<Todo>),
    TodoDeleted(Todo),
    TodoFound(Todo),
}

#[derive(thiserror::Error, Debug)]
pub enum TodoError {
    #[error("Todo not found: {0}")]
    TodoNotFound(usize),
}

trait_set!(pub trait TodoService<F> = Send + Sync + Service<TodoRequest, Response = TodoResponse, Future = F>);
