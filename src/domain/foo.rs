use std::str::FromStr;
use uuid::{Uuid};

pub fn name_space() -> Uuid {
    Uuid::from_str("c5941853-26f0-41e9-9b09-1d7e0bcd5891").unwrap()
}

#[derive(Debug)]
pub struct FooId {
    id: Uuid,
}

impl FooId {
    pub fn new(name: &String) -> Self {
        let uuid = Uuid::new_v5(&name_space(), &name.as_bytes());
        FooId { id: uuid }
    }
    pub fn reconstitute(uuid: &String) -> Self {
        let uuid_result = Uuid::from_str(uuid);
        match uuid_result {
            Ok(uuid) => {
                FooId { id: uuid }
            }
            Err(_) => {
                panic!("Fail to instantiate Foo for id: [{:?}]", uuid);
            }
        }
    }
    pub fn to_uuid(&self) -> Uuid {
        self.id
    }
}

pub struct Foo {
    name: String,
    number_of_years_optional: Option<i64>,
}

impl Foo {
    pub fn new(name: String, number_of_years_optional: Option<i64>) -> Self {
        Foo { name, number_of_years_optional }
    }

    pub fn reconstitute(name: String, number_of_years: Option<i64>) -> Self {
        Foo {
            name,
            number_of_years_optional: number_of_years,
        }
    }

    pub fn id(&self) -> FooId {
        FooId::new(&self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number_of_years(&self) -> Option<i64> {
        self.number_of_years_optional
    }

    pub fn increment_year(&mut self) {
        if let Some(mut number_of_years) = self.number_of_years_optional {
            number_of_years += 1;
            self.number_of_years_optional = Some(number_of_years);
        } else {
            self.number_of_years_optional = Some(1);
        }
    }
}
