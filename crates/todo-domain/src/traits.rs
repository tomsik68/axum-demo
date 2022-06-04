use std::future::Future;

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct Todo {
    pub id: usize,
    pub text: String,
}

#[derive(thiserror::Error, Debug)]
pub enum TodoError {
    #[error("Todo not found: {0}")]
    TodoNotFound(usize),
}

pub trait SencFut<T, E>: Future<Output = Result<T, E>> + Send + Sync + 'static {}
impl<T, E, F> SencFut<T, E> for F where F: Future<Output = Result<T, E>> + Send + Sync + 'static {}

pub trait TodoService {
    type AddTodoFuture: SencFut<Todo, TodoError>;
    fn add_todo(&self, text: String) -> Self::AddTodoFuture;

    type ListTodosFuture: SencFut<Vec<Todo>, TodoError>;
    fn list_todos(&self) -> Self::ListTodosFuture;

    type GetTodoFuture: SencFut<Todo, TodoError>;
    fn get_todo(&self, id: usize) -> Self::GetTodoFuture;

    type DeleteTodoFuture: SencFut<Todo, TodoError>;
    fn delete_todo(&self, id: usize) -> Self::DeleteTodoFuture;
}
