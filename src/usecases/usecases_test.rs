#[cfg(test)]
mod tests {
    use std::env;
    use futures::executor::block_on;
    use sqlx::postgres::PgPoolOptions;
    use crate::{App};
    use crate::domain::foo::Foo;

    pub fn before() -> App {
        let app = App::new();
        let database_url_result = env::var("DATABASE_URL");
        if database_url_result.is_err() {
            panic!("DATABASE_URL env var has not been set");
        }
        let pool = block_on(PgPoolOptions::new().max_connections(5).connect(&database_url_result.unwrap())).unwrap();
        block_on(sqlx::query("DELETE FROM foo").execute(&pool)).unwrap();
        return app;
    }

    #[test]
    pub fn launch_all_tests_with_db() {
        it_should_create_a_new_foo_in_database();
        it_should_find_all_foo();
        it_should_increment_year_of_foo();
    }

    pub fn it_should_create_a_new_foo_in_database() {
        // Given
        let app = before();
        let name = "Bob";

        // When
        let id = app.create_foo.execute(name.to_string(), None);

        // Then
        let foo_found_opt = app.foo_repository.find_by_id(&id);
        assert_eq!(foo_found_opt.is_some(), true);
        let foo_found = foo_found_opt.unwrap();
        assert_eq!(foo_found.id().to_uuid(), id.to_uuid());
    }

    pub fn it_should_find_all_foo() {
        // Given
        let app = before();
        let foo1 = Foo::new("name".to_string(), Some(4));
        let foo2 = Foo::new("last_name".to_string(), None);
        app.foo_repository.save(&foo1);
        app.foo_repository.save(&foo2);

        // When
        let foos = app.find_all_foo.execute();

        // Then
        assert_eq!(foos.len(), 2);
    }

    pub fn it_should_increment_year_of_foo() {
        // Given
        let app = before();
        let foo1 = Foo::new("name".to_string(), Some(4));
        app.foo_repository.save(&foo1);

        // When
        let foo_result = app.increment_year_of_foo.execute(foo1.id());

        // Then
        assert_eq!(foo_result.is_ok(), true);
        let foo = foo_result.unwrap();
        assert_eq!(foo.number_of_years().unwrap(), 5);
    }
}
