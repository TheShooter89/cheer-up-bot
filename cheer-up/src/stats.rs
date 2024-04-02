use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::{requests::ResponseResult, types::Chat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_videonotes: i32,
    pub users: Vec<UserStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub username: String,
    pub videonotes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatsBody<T> {
    stats: T,
}

pub async fn get_stats() -> ResponseResult<Stats> {
    let client = Client::new();

    let notes_stats = client
        .get(format!("http://0.0.0.0:1989/api/stats"))
        .send()
        .await?
        .json::<StatsBody<Stats>>()
        .await?;

    Ok(notes_stats.stats)
}

pub async fn get_user_stats_by_id(user_id: &i64) -> ResponseResult<UserStats> {
    let client = Client::new();

    let user_stats = client
        .get(format!("http://0.0.0.0:1989/api/stats/user/{}", user_id))
        .send()
        .await?
        .json::<StatsBody<UserStats>>()
        .await?;

    Ok(user_stats.stats)
}
