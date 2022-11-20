use futures::executor::block_on;
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgRow;
use crate::domain::foo::{Foo, FooId};
use crate::domain::foo_repository::FooRepository;

pub struct FooRepositorySqlImpl {
    pool: Pool<Postgres>,
}

impl FooRepositorySqlImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        FooRepositorySqlImpl { pool }
    }

    fn to_foo(&self, row: PgRow) -> Foo {
        let name: String = row.try_get("name").unwrap();
        let number_of_years_optional: Option<i64> = row.try_get("number_of_years").unwrap();
        Foo::reconstitute(name, number_of_years_optional)
    }
}

impl FooRepository for FooRepositorySqlImpl {
    fn save(&self, foo: &Foo) {
        // for rust last version, it is not possible to use async function with trait
        block_on(sqlx::query("INSERT INTO foo(id, name, number_of_years) values($1, $2, $3)")
            .bind(foo.id().to_uuid().to_string())
            .bind(foo.name())
            .bind(foo.number_of_years())
            .execute(&self.pool)).unwrap();
    }

    fn update(&self, foo: &Foo) {
        // for rust last version, it is not possible to use async function with trait
        block_on(sqlx::query("UPDATE foo set number_of_years = $1 where id = $2")
            .bind(foo.number_of_years())
            .bind(foo.id().to_uuid().to_string())
            .execute(&self.pool)).unwrap();
    }

    fn find_by_id(&self, id: &FooId) -> Option<Foo> {
        block_on(sqlx::query("select * from foo where id = $1")
            .bind(id.to_uuid().to_string())
            .map(|row: PgRow| {
                self.to_foo(row)
            })
            .fetch_optional(&self.pool)).unwrap()
    }

    fn find_all(&self) -> Vec<Foo> {
        block_on(sqlx::query("select * from foo")
            .map(|row: PgRow| {
                self.to_foo(row)
            })
            .fetch_all(&self.pool)).unwrap()
    }
}
