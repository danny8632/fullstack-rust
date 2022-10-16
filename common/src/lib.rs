use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestStruct {
    pub name: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Register {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String
}