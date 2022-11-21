#[cfg(test)]
mod tests {
    use crate::domain::foo::{Foo, FooId};

    #[test]
    #[should_panic]
    pub fn it_should_panic_when_reconstitute_id_with_wrong_uuid() {
        // Given
        let wrong_uuid = "not_an_uuid".to_string();

        // When
        FooId::reconstitute(&wrong_uuid);

        // Then
        // should panic!
    }

    #[test]
    pub fn it_should_create_a_foo_correctly() {
        // Given
        let name = String::from("Bob");
        let number_of_years = None;

        // When
        let foo = Foo::new(name, number_of_years);

        // Then
        assert_eq!(foo.id().to_uuid(), FooId::new(&name).to_uuid());
        assert_eq!(foo.name(), name);
        assert_eq!(foo.number_of_years(), None);
    }

    #[test]
    pub fn it_should_increment_year_by_one() {
        // Given
        let name = String::from("Bob");
        let number_of_years = Some(4);
        let mut foo = Foo::new(name.clone(), number_of_years);

        // When
        foo.increment_year();

        // Then
        assert_eq!(foo.number_of_years().is_some(), true);
        assert_eq!(foo.number_of_years().unwrap(), 5);
    }

    #[test]
    pub fn it_should_init_year_to_one_when_none() {
        // Given
        let name = String::from("Bob");
        let number_of_years = None;
        let mut foo = Foo::new(name.clone(), number_of_years);

        // When
        foo.increment_year();

        // Then
        assert_eq!(foo.number_of_years().is_some(), true);
        assert_eq!(foo.number_of_years().unwrap(), 1);
    }

}
