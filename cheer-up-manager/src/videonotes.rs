use dotenvy as dotenv;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::{
    net::Download,
    requests::{Requester, ResponseResult},
    types::{Chat, VideoNote},
    Bot,
};
use tokio::fs;

use crate::user::*;
use crate::utils::{create_user_folder, get_vnote_filename};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub file_name: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct DeleteNote {
    id: Option<i64>,
    user_id: Option<i64>,
    file_name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct UpdateNote {
    id: Option<i64>,
    user_id: Option<i64>,
    file_name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewNote {
    user_id: i64,
    file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteBody<T> {
    note: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteListBody<T> {
    notes: Vec<T>,
}

pub async fn upload_vnote(bot: &Bot, videonote: &VideoNote, chat: &Chat) -> ResponseResult<()> {
    let folder_path = create_user_folder(&chat).await?;
    println!("created user folder path is: {}", folder_path);

    let vnote_file = bot.get_file(&videonote.file.id).await?;
    let vnote_out_path = format!("{}/{}", folder_path, get_vnote_filename(&videonote));

    let mut output_file = fs::File::create(&vnote_out_path).await?;

    bot.download_file(&vnote_file.path, &mut output_file)
        .await?;

    Ok(())
}

pub async fn save_vnote_to_db(vnote: &VideoNote, author: &Chat) -> ResponseResult<()> {
    let client = Client::new();

    let vnote_author = match get_user_by_telegram_id(author).await {
        Ok(vnote) => vnote,
        Err(_) => save_user_to_db(&author).await?,
    };

    let output_file_name = format!("{}.mpeg", vnote.file.id);
    let new_note = NewNote {
        user_id: vnote_author.id,
        file_name: output_file_name,
    };
    println!("new_note is: {:#?}", new_note);

    let resp = client
        .post("http://0.0.0.0:1989/api/notes")
        .json::<NewNote>(&new_note)
        .send()
        .await?;

    println!("resp is: {:#?}", resp);

    Ok(())
}

pub async fn get_vnote_list_from_db(author: &Chat) -> ResponseResult<Vec<Note>> {
    let client = Client::new();

    let vnote_author = match get_user_by_telegram_id(author).await {
        Ok(user) => user,
        Err(_) => save_user_to_db(&author).await?,
    };

    let vnote_list = client
        .get(format!(
            "http://0.0.0.0:1989/api/notes/user/{}",
            vnote_author.id
        ))
        .send()
        .await?
        .json::<NoteListBody<Note>>()
        .await?;

    Ok(vnote_list.notes)
}

pub async fn delete_all_user_vnotes(author: &Chat) -> ResponseResult<()> {
    let client = Client::new();

    let vnote_author = match get_user_by_telegram_id(author).await {
        Ok(user) => user,
        Err(_) => save_user_to_db(&author).await?,
    };

    let _deleted_vnote = client
        .delete(format!(
            "http://0.0.0.0:1989/api/notes/user/{}",
            vnote_author.id
        ))
        .send()
        .await?
        .json::<NoteBody<String>>()
        .await?;

    Ok(())
}
