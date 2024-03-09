use std::io::Error;

use log;
use teloxide::{
    net::Download,
    payloads::SendMessageSetters,
    prelude::*,
    types::{InputFile, ParseMode},
    RequestError,
};
use tokio::fs;

mod templates;
use templates::Templates;

mod commands;
use commands::*;

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    VideoNote,
    Text,
    Photo,
    Video,
    Voice,
    Audio,
    Document,
    Unknown,
}

impl MessageType {
    pub fn from_msg(msg: &Message) -> MessageType {
        if let Some(_) = msg.video_note() {
            return MessageType::VideoNote;
        }

        if let Some(_) = msg.text() {
            return MessageType::Text;
        }

        if let Some(_) = msg.photo() {
            return MessageType::Photo;
        }

        if let Some(_) = msg.video() {
            return MessageType::Video;
        }

        if let Some(_) = msg.voice() {
            return MessageType::Voice;
        }

        if let Some(_) = msg.audio() {
            return MessageType::Audio;
        }

        if let Some(_) = msg.document() {
            return MessageType::Document;
        }

        MessageType::Unknown
    }

    pub fn name(&self) -> &str {
        match self {
            MessageType::VideoNote => "videonote",
            MessageType::Text => "text",
            MessageType::Photo => "photo",
            MessageType::Video => "video",
            MessageType::Voice => "voice",
            MessageType::Audio => "audio",
            MessageType::Document => "document",
            MessageType::Unknown => "unknown",
        }
    }
}

async fn handle_input(bot: Bot, msg: Message) -> ResponseResult<()> {
    let message_type = MessageType::from_msg(&msg);
    println!("Message type you sent: {:?}", message_type);

    let chat_id = msg.chat.id;

    match message_type {
        MessageType::VideoNote => {
            println!("received video note");
            let vnote = msg.video_note().unwrap();
            let vnote_file_id = vnote.file.id.clone();

            match download_vnote(&bot, &vnote_file_id, chat_id).await {
                Ok(res) => println!("vnote succefully saved: {:?}", res),
                Err(e) => println!("error while saving video note: {:?}", e),
            }
            Ok(())
        }
        MessageType::Text => {
            let command = Command::parse_str(msg.text().unwrap_or("none"));

            println!("Text you sent: {:?}", msg.text().unwrap_or("NONE"));

            if let Some(cmd) = command {
                println!("Command you sent: {:?}", cmd);
                handle_commands(bot.clone(), cmd, msg.clone()).await?;

                return Ok(());
            }

            start_command(bot.clone(), msg.clone()).await?;

            Ok(())
        }
        _ => {
            bot.send_message(
                chat_id,
                Templates::UnsupportedInputPage(message_type.name().to_string()).render(),
            )
            .parse_mode(ParseMode::Html)
            .await?;

            Ok(())
        }
    }
}

async fn download_vnote(bot: &Bot, file_id: &str, chat_id: ChatId) -> Result<(), RequestError> {
    let file = bot.get_file(file_id).await?;

    // let folder_name = chat_id.to_string();
    let folder_name = "../_common_data/videonotes";
    let file_name = format!("{}/{}.mpeg", folder_name, file_id);

    let mut output_file = fs::File::create(file_name.clone()).await?;

    bot.download_file(&file.path, &mut output_file).await?;

    bot.send_video_note(chat_id, InputFile::file(file_name))
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    dotenvy::dotenv().ok();

    let bot = Bot::from_env();

    teloxide::repl(bot, handle_input).await;
    Ok(())
}
