#![feature(proc_macro_hygiene, decl_macro)]

use diesel::result::Error;
use model::{Task, Todo};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

mod db;
mod model;
mod schema;

#[post("/todo", data = "<todo>", format = "json")]
fn create(todo: Json<Todo>, connection: db::Connection) -> Result<Json<Task>, Status> {
    Task::create(todo.into_inner(), &connection)
        .map(Json)
        .map_err(error_status)
}

#[get("/todo/<id>")]
fn get(id: i32, connection: db::Connection) -> Result<Json<Task>, Status> {
    Task::get(id, &connection).map(Json).map_err(error_status)
}

#[put("/todo/<id>", data = "<todo>", format = "json")]
fn update(id: i32, todo: Json<Todo>, connection: db::Connection) -> Result<Json<Task>, Status> {
    Task::update(id, todo.into_inner(), &connection)
        .map(Json)
        .map_err(error_status)
}

#[get("/todos")]
fn all(connection: db::Connection) -> Result<Json<Vec<Task>>, Status> {
    Task::all(&connection).map(Json).map_err(error_status)
}

#[delete("/todo/<id>")]
fn delete(id: i32, connection: db::Connection) -> Result<Json<Task>, Status> {
    Task::delete(id, &connection)
        .map(Json)
        .map_err(error_status)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/api/v1/", routes![all, get, create, delete, update])
        .launch();
}
