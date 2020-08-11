#![feature(proc_macro_hygiene, decl_macro)]

use model::{Task, Todo};
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
fn create(todo: Json<Todo>, connection: db::Connection) -> Json<bool> {
    Json(Task::create(todo.into_inner(), &connection))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/api/v1/", routes![create])
        .launch();
}
