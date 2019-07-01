use rocket_contrib::databases::postgres;
use serde::{Deserialize, Serialize};

const DB_ERROR_CODE: i32 = 5000;
const USER_EXISTS_ERROR_CODE: i32 = 5001;
const LOGIN_ERROR_CODE: i32 = 5003;
const NO_USER_ERROR_CODE: i32 = 5004;
const EDIT_ERROR_CODE: i32 = 5005;
const DELETE_ERROR_CODE: i32 = 5007;
const WRONG_FORMAT_ERROR_CODE: i32 = 5008;
const WRONG_PATH_ERROR_CODE: i32 = 5009;

const DB_ERROR: &str = "Database error";
const USER_EXISTS_ERROR: &str = "The user already exists";
const LOGIN_ERROR: &str = "Username or password is incorrect";
const NO_USER_ERROR: &str = "The provided user does not exist";
const EDIT_ERROR: &str = "User or todo does not exist";
const DELETE_ERROR: &str = "The todo or user does not exist";
const WRONG_FORMAT_ERROR: &str = "Invalid JSON format";
const WRONG_PATH_ERROR: &str = "Invalid path";

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorTypes {
    WrongFormat,
    WrongPath,
    Login,
    NoUser,
    Edit,
    Delete,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    code: i32,
    error: String,
}

pub fn get_database_error(error: postgres::Error) -> ResponseError {
    error.code().map_or(
        ResponseError {
            code: DB_ERROR_CODE,
            error: DB_ERROR.to_owned(),
        },
        |code| match code.code() {
            "23503" => ResponseError {
                code: NO_USER_ERROR_CODE,
                error: NO_USER_ERROR.to_owned(),
            },
            "23505" => ResponseError {
                code: USER_EXISTS_ERROR_CODE,
                error: USER_EXISTS_ERROR.to_owned(),
            },
            _ => ResponseError {
                code: DB_ERROR_CODE,
                error: DB_ERROR.to_owned(),
            },
        },
    )
}

pub fn get_query_error(error_type: ErrorTypes) -> ResponseError {
    match error_type {
        ErrorTypes::WrongFormat => ResponseError {
            code: WRONG_FORMAT_ERROR_CODE,
            error: WRONG_FORMAT_ERROR.to_owned(),
        },
        ErrorTypes::WrongPath => ResponseError {
            code: WRONG_PATH_ERROR_CODE,
            error: WRONG_PATH_ERROR.to_owned(),
        },
        ErrorTypes::Login => ResponseError {
            code: LOGIN_ERROR_CODE,
            error: LOGIN_ERROR.to_owned(),
        },
        ErrorTypes::NoUser => ResponseError {
            code: NO_USER_ERROR_CODE,
            error: NO_USER_ERROR.to_owned(),
        },
        ErrorTypes::Edit => ResponseError {
            code: EDIT_ERROR_CODE,
            error: EDIT_ERROR.to_owned(),
        },
        ErrorTypes::Delete => ResponseError {
            code: DELETE_ERROR_CODE,
            error: DELETE_ERROR.to_owned(),
        }
    }
}
