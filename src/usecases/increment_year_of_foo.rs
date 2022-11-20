use std::rc::Rc;
use crate::domain::errors::FooNotFound;
use crate::domain::foo::{Foo, FooId};
use crate::domain::foo_repository::FooRepository;

pub struct IncrementYearOfFoo {
    foo_repository: Rc<dyn FooRepository>,
}

impl IncrementYearOfFoo {
    pub fn new(foo_repository: Rc<dyn FooRepository>) -> Self {
        IncrementYearOfFoo { foo_repository }
    }

    pub fn execute(&self, id: FooId) -> Result<Foo, FooNotFound> {
        let foo_option = self.foo_repository.find_by_id(&id);
        if let Some(mut foo) = foo_option {
            foo.increment_year();
            self.foo_repository.update(&foo);
            Ok(foo)
        } else {
            Err(FooNotFound::new(&id))
        }
    }
}
