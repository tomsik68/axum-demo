use std::sync::Arc;
use std::{marker::PhantomData, sync::Mutex};

use todo_domain::{InMemoryTodo, Todo, TodoService};
use typed_builder::TypedBuilder;

trait_set::trait_set! {pub trait Senc = Send + Sync + 'static}

#[derive(TypedBuilder, degeneric_macros::Degeneric)]
#[degeneric(trait = "pub trait ContainerTrait")]
pub struct Container<TodoFactory, Todo>
where
    Todo: TodoService + Senc,
    TodoFactory: TodoFact<Todo> + Senc + Clone,
{
    todo_factory: TodoFactory,

    #[builder(default)]
    _todo: PhantomData<Todo>,
}

impl<TodoFactory, Todo> Clone for Container<TodoFactory, Todo>
where
    TodoFactory: Clone + TodoFact<Todo> + Senc,
    Todo: TodoService + Senc,
{
    fn clone(&self) -> Self {
        Self {
            todo_factory: self.todo_factory.clone(),
            _todo: PhantomData,
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
