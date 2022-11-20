use crate::domain::foo::{Foo, FooId};

pub trait FooRepository {
    fn save(&self, foo: &Foo);
    fn update(&self, foo: &Foo);
    fn find_by_id(&self, id: &FooId) -> Option<Foo>;
    fn find_all(&self) -> Vec<Foo>;
}
