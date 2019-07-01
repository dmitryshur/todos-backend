#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_map_or_else)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate serde;

mod errors;
mod request_types;

use errors::{get_database_error, get_query_error, ErrorTypes, ResponseError};
use request_types::{CreateRequest, DeleteRequest, EditRequest, GetRequest, LoginRequest, RegisterRequest};

use rocket::Request;
use rocket::response::status;
use rocket_contrib::databases::postgres;
use rocket_contrib::json::{Json, JsonValue};

use serde::Serialize;

use chrono::prelude::*;
use chrono::DateTime;

#[derive(Serialize, Debug)]
pub struct Todo {
    id: i32,
    title: String,
    body: String,
    done: bool,
    creation_time: DateTime<Local>,
}

#[database("todo")]
struct TodosDbConnection(postgres::Connection);

#[catch(422)]
fn wrong_format(_req: &Request) -> Json<ResponseError> {
    Json(get_query_error(ErrorTypes::WrongFormat))
}

#[post("/register", format = "json", data = "<user>")]
fn register(
    connection: TodosDbConnection,
    user: Json<RegisterRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.query(
        "INSERT INTO users (username, password) VALUES ($1, crypt($2, gen_salt('bf', 8))) RETURNING id",
        &[&user.username, &user.password],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|_rows| {
        Ok(status::Accepted(Some(json!({}))))
    })
}

#[post("/login", format = "json", data = "<user>")]
fn login(
    connection: TodosDbConnection,
    user: Json<LoginRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.query(
        "SELECT id FROM users WHERE username=$1 AND password=crypt($2, password)",
        &[&user.username, &user.password],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|rows| {
        if rows.is_empty() {
            return Err(status::BadRequest(Some(Json(get_query_error(ErrorTypes::Login)))));
        }

        let id: i32 = rows.get(0).get("id");
        Ok(status::Accepted(Some(json!({"id": id}))))
    })
}

#[post("/create", format = "json", data = "<todo>")]
fn create(
    connection: TodosDbConnection,
    todo: Json<CreateRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.query(
        "INSERT INTO todos (title, body, user_id) VALUES ($1, $2, $3) RETURNING id, creation_time",
        &[&todo.title, &todo.body, &todo.user_id],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|rows| {
        let created_todo = rows.get(0);
        let id: i32 = created_todo.get("id");
        let creation_time: DateTime<Local> = created_todo.get("creation_time");

        Ok(status::Accepted(Some(json!({"id": id, "creation_time": creation_time}))))
    })
}

#[post("/get", format = "json", data = "<todos>")]
fn get(
    connection: TodosDbConnection,
    todos: Json<GetRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.query(
        "SELECT todos.id, title, body, done, creation_time FROM todos INNER JOIN users ON todos.user_id = users.id WHERE user_id = $1 OFFSET $2 LIMIT $3",
        &[&todos.user_id, &todos.offset, &todos.count],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|rows| {
        let todos: Vec<Todo> = rows
            .iter()
            .map(|row| Todo {
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                done: row.get("done"),
                creation_time: row.get("creation_time"),
            })
            .collect();
        Ok(status::Accepted(Some(json!({"todos": todos}))))
    })
}

#[post("/edit", format = "json", data = "<todo>")]
fn edit(
    connection: TodosDbConnection,
    todo: Json<EditRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.execute(
        "UPDATE todos SET title = COALESCE($1, title), body = COALESCE($2, body), done = COALESCE($3, done) WHERE id = $4 AND user_id = $5",
        &[&todo.title, &todo.body, &todo.done, &todo.todo_id, &todo.user_id],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|count| {
        if count == 0 {
            return Err(status::BadRequest(Some(Json(get_query_error(ErrorTypes::Edit)))));
        }

        Ok(status::Accepted(Some(json!({}))))
    })
}

#[post("/delete", format = "json", data = "<todo>")]
fn delete(
    connection: TodosDbConnection,
    todo: Json<DeleteRequest>,
) -> Result<status::Accepted<JsonValue>, status::BadRequest<Json<ResponseError>>> {
    connection.0.execute(
        "DELETE FROM todos WHERE id = $1 AND user_id = $2",
        &[&todo.todo_id, &todo.user_id],
    ).map_err(|error| {
        status::BadRequest(Some(Json(get_database_error(error))))
    }).and_then(|count| {
        if count == 0 {
            return Err(status::BadRequest(Some(Json(get_query_error(ErrorTypes::Delete)))));
        }

        Ok(status::Accepted(Some(json!({}))))
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![register, login, create, get, edit, delete])
        .register(catchers![wrong_format])
        .attach(TodosDbConnection::fairing())
        .launch();
}

