use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestStruct {
    pub name: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_on: NaiveDate,
    pub last_login: NaiveDate
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub email: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Register {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String
}