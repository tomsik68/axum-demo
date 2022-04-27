use trait_set::trait_set;
use typed_builder::TypedBuilder;

trait_set!(pub trait Factory<T> = Send + Sync + 'static + Fn() -> T);

#[derive(degeneric_macros::Degeneric, Clone, TypedBuilder)]
#[degeneric(trait = "pub trait ContainerTrait")]
pub struct Container {}

trait_set::trait_set!(pub trait ThreadSafeContainer = ContainerTrait + Send + Sync + Clone + 'static);

pub(crate) fn create_production_container() -> impl ThreadSafeContainer {
    Container::builder().build()
}
