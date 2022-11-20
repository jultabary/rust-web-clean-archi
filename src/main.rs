#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
use std::env;
use std::rc::Rc;
use futures::executor::block_on;
use log::LevelFilter;
use rocket::{catchers, routes};
use sqlx::postgres::PgPoolOptions;
use crate::domain::foo_repository::FooRepository;
use crate::infrastructure::persistence::foo_repository_sql_impl::FooRepositorySqlImpl;
use crate::usecases::create_foo::CreateFoo;
use crate::usecases::find_all_foo::FindAllFoo;
use crate::usecases::increment_year_of_foo::IncrementYearOfFoo;
use crate::infrastructure::route_http::routes_http::{create_foo, get_all_foos, increment_year_of_foo, not_found};
use crate::logger::SimpleLogger;

mod domain;
mod usecases;
mod infrastructure;
mod logger;

pub struct App {
    foo_repository: Rc<dyn FooRepository>,
    create_foo: CreateFoo,
    find_all_foo: FindAllFoo,
    increment_year_of_foo: IncrementYearOfFoo,
}

impl App {
    pub fn new() -> Self {
        let database_url_result = env::var("DATABASE_URL");
        if database_url_result.is_err() {
            panic!("DATABASE_URL env var has not been set");
        }
        let pool = block_on(PgPoolOptions::new().max_connections(5).connect(&database_url_result.unwrap())).unwrap();
        let foo_repository = Rc::new(FooRepositorySqlImpl::new(pool));
        App {
            foo_repository: foo_repository.clone(),
            create_foo: CreateFoo::new(foo_repository.clone()),
            find_all_foo: FindAllFoo::new(foo_repository.clone()),
            increment_year_of_foo: IncrementYearOfFoo::new(foo_repository.clone()),
        }
    }
}

unsafe impl Sync for App {}

unsafe impl Send for App {}

static LOGGER: SimpleLogger = SimpleLogger {};

#[rocket::main]
async fn main() {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    let app = App::new();
    let _server = rocket::build()
        .manage(app)
        .register("/", catchers![not_found])
        .mount("/", routes![create_foo, get_all_foos, increment_year_of_foo]).launch().await;
}
