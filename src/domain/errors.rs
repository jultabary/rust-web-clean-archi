use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::domain::foo::FooId;

#[derive(Debug)]
pub struct FooNotFound {
    id: String,
}

impl FooNotFound {
    pub fn new(id: &FooId) -> Self {
        FooNotFound { id: id.to_uuid().to_string() }
    }
}

impl Error for FooNotFound {}

impl Display for FooNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo with id [{:?}] not found.", self.id)
    }
}

