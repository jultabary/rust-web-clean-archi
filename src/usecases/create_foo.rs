use std::rc::Rc;
use crate::domain::foo::{Foo, FooId};
use crate::domain::foo_repository::FooRepository;

pub struct CreateFoo {
    foo_repository: Rc<dyn FooRepository>,
}

impl CreateFoo {
    pub fn new(foo_repository: Rc<dyn FooRepository>) -> Self {
        CreateFoo { foo_repository }
    }

    pub fn execute(&self, name: String, number_of_year_optional: Option<i64>) -> FooId {
        let foo = Foo::new(name.clone(), number_of_year_optional);
        self.foo_repository.save(&foo);
        foo.id()
    }
}
