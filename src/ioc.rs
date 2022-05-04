use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use std::{marker::PhantomData, sync::Mutex};

use crate::todo_service::TodoRequest;
use crate::{
    in_memory_todo::InMemoryTodo,
    todo_service::Todo,
    todo_service::{TodoError, TodoResponse, TodoService},
};
use tower::{MakeService, Service};
use typed_builder::TypedBuilder;

trait_set::trait_set! {pub trait Senc = Send + Sync + 'static}
trait_set::trait_set! {pub trait TodoFuture = Future<Output = Result<TodoResponse, TodoError>> + Senc}

#[derive(TypedBuilder, degeneric_macros::Degeneric)]
#[degeneric(trait = "pub trait ContainerTrait")]
pub struct Container<TodoFactory, Todo, TodoFut>
where
    Todo: TodoService<TodoFut> + Senc,
    TodoFut: TodoFuture,
    TodoFactory: TodoFact<Todo> + Senc + Clone,
{
    todo_factory: TodoFactory,

    #[builder(default)]
    _todo: PhantomData<Todo>,

    #[builder(default)]
    _todo_fut: PhantomData<TodoFut>,
}

impl<TodoFactory, Todo, TodoFut> Clone for Container<TodoFactory, Todo, TodoFut>
where
    TodoFactory: Clone + TodoFact<Todo> + Senc,
    TodoFut: TodoFuture,
    Todo: TodoService<TodoFut> + Senc,
{
    fn clone(&self) -> Self {
        Self {
            todo_factory: self.todo_factory.clone(),
            _todo: PhantomData,
            _todo_fut: PhantomData,
        }
    }
}

trait_set::trait_set!(pub trait ThreadSafeContainer = ContainerTrait + Senc + Clone);

#[derive(Default, Clone)]
pub struct TodoFactoryImpl(Arc<Mutex<Vec<Todo>>>);

pub trait TodoFact<T> {
    fn create(&self) -> T;
}

impl TodoFact<InMemoryTodo> for TodoFactoryImpl {
    fn create(&self) -> InMemoryTodo {
        InMemoryTodo::new(Arc::clone(&self.0))
    }
}

pub(crate) fn create_production_container() -> impl ThreadSafeContainer {
    Container::builder()
        .todo_factory(TodoFactoryImpl::default())
        .build()
}
