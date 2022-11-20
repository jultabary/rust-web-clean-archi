use std::rc::Rc;
use crate::domain::foo::Foo;
use crate::domain::foo_repository::FooRepository;

pub struct FindAllFoo {
    foo_repository: Rc<dyn FooRepository>,
}

impl FindAllFoo {
    pub fn new(foo_repository: Rc<dyn FooRepository>) -> Self {
        FindAllFoo { foo_repository }
    }

    pub fn execute(&self) -> Vec<Foo> {
        self.foo_repository.find_all()
    }
}
