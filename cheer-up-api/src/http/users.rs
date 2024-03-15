use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    telegram_id: u64,
    username: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct UpdateUser {
    id: Option<u64>,
    telegram_id: Option<u64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    telegram_id: u64,
    username: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}
