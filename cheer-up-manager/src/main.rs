use std::io::Error;

use log;
use teloxide::{
    net::Download, prelude::*, types::InputFile, utils::command::BotCommands, RequestError,
};
use tokio::fs;

mod templates;
use templates::Templates;

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
    pub fn from_msg(msg: Message) -> MessageType {
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
}

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "CheerUp Bot starting page")]
    Start,
    #[command(description = "List all uploaded video notes")]
    List,
    #[command(description = "Erase all video notes")]
    Erase,
    #[command(description = "Show help and available commands")]
    Help,
    #[command(description = "Show credits and code repo links")]
    Credits,
}

async fn handle_commands(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Start => println!("Start Command"),
        Command::List => println!("List Command"),
        Command::Erase => println!("Erase Command"),
        Command::Help => println!("Help Command"),
        Command::Credits => println!("Credits Command"),
    }

    Ok(())
}

async fn download_vnote(bot: &Bot, file_id: &str, chat_id: ChatId) -> Result<(), RequestError> {
    let file = bot.get_file(file_id).await?;

    let mut output_file = fs::File::create("_EXTRA/test.mpeg").await?;

    bot.download_file(&file.path, &mut output_file).await?;

    bot.send_video_note(chat_id, InputFile::file("../_common_data//test.mpeg"))
        .await;

    Ok(())
}

#[tokio::main]
async fn main() {
    log::info!("Starting throw dice bot...");
    dotenvy::dotenv().ok();

    let bot = Bot::from_env();

    Command::repl(bot, handle_commands).await;
}
