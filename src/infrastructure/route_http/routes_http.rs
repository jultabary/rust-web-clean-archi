use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Request, State};
use crate::App;
use crate::domain::foo::{FooId};

#[derive(Deserialize)]
pub struct CreateFooRequest {
    name: String,
    number_of_years: Option<i64>,
}

#[derive(Serialize)]
pub struct FooIdCreatedResponse {
    id: String,
}

#[post("/foo", format = "json", data = "<foo_create_request>")]
pub fn create_foo(foo_create_request: Json<CreateFooRequest>, app: &State<App>) -> Json<FooIdCreatedResponse> {
    let response = app.create_foo.execute(foo_create_request.name.to_string(), foo_create_request.number_of_years);
    Json(FooIdCreatedResponse { id: response.to_uuid().to_string() })
}

#[derive(Serialize)]
pub struct FooResponse {
    id: String,
    name: String,
    number_of_years: Option<i64>,
}

#[get("/foo", format = "json")]
pub fn get_all_foos(app: &State<App>) -> Json<Vec<FooResponse>> {
    let foos = app.find_all_foo.execute();
    let response = foos.iter()
        .map(|foo| { FooResponse { id: foo.id().to_uuid().to_string(), name: foo.name().to_string(), number_of_years: foo.number_of_years() } })
        .collect::<Vec<FooResponse>>();
    Json(response)
}

#[put("/foo/<id>", format = "json")]
pub fn increment_year_of_foo(id: String, app: &State<App>) -> Result<Json<FooResponse>, Status> {
    let foo_updated_result = app.increment_year_of_foo.execute(FooId::reconstitute(&id));
    match foo_updated_result {
        Ok(foo_updated) => {
            Ok(Json(FooResponse { id: foo_updated.id().to_uuid().to_string(), name: foo_updated.name().to_string(), number_of_years: foo_updated.number_of_years() }))
        }
        Err(_) => {
            Err(Status::NotFound)
        }
    }
}

#[catch(404)]
pub fn not_found(_req: &Request) -> String {
    "Foo not found".to_string()
}
