use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateRequest {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct GetRequest {
    pub user_id: i32,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct EditRequest {
    pub user_id: i32,
    pub todo_id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub done: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    pub todo_id: i32,
    pub user_id: i32,
}
