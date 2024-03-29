use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::{requests::ResponseResult, types::Chat};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserId(i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateUser {
    id: Option<i64>,
    telegram_id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    telegram_id: i64,
    username: String,
    first_name: String,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListBody<T> {
    pub users: Vec<T>,
}

pub async fn save_user_to_db(user: &Chat) -> ResponseResult<User> {
    let new_user = NewUser {
        telegram_id: user.id.0,
        username: user
            .username()
            .unwrap_or(&user.id.0.to_string())
            .to_string(),
        first_name: user
            .username()
            .unwrap_or(&user.id.0.to_string())
            .to_string(),
        last_name: match user.last_name() {
            Some(name) => Some(name.to_string()),
            None => None,
        },
    };

    let client = Client::new();

    let resp = client
        .post("http://0.0.0.0:1989/api/users")
        .json::<NewUser>(&new_user)
        .send()
        .await?;
    let res_json = resp.json::<UserBody<User>>().await?.user;
    println!("resp.json() is: {:#?}", res_json);

    Ok(res_json)
}

pub async fn get_user_by_telegram_id(user: &Chat) -> ResponseResult<User> {
    let client = Client::new();

    let vnote_author = client
        .get(format!(
            "http://0.0.0.0:1989/api/users/name/{}",
            user.username().unwrap_or("guest")
        ))
        .send()
        .await?
        .json::<UserBody<User>>()
        .await?;

    Ok(vnote_author.user)
}

pub async fn get_user_by_id(user_id: &i64) -> ResponseResult<User> {
    let client = Client::new();

    let vnote_author = client
        .get(format!("http://0.0.0.0:1989/api/users/{}", user_id))
        .send()
        .await?
        .json::<UserBody<User>>()
        .await?;

    Ok(vnote_author.user)
}
