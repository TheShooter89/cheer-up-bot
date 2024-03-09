use std::io::Error;

use log;
use teloxide::{
    net::Download,
    payloads::SendMessageSetters,
    prelude::*,
    types::{InputFile, ParseMode},
    utils::command::{self, BotCommands},
    RequestError,
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

impl Command {
    pub fn parse_str(cmd: &str) -> Option<Command> {
        match cmd {
            "/start" => Some(Command::Start),
            "/list" => Some(Command::List),
            "/erase" => Some(Command::Erase),
            "/help" => Some(Command::Help),
            "/credits" => Some(Command::Credits),
            _ => None,
        }
    }
}

async fn handle_commands(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Start => start_command(bot, msg).await?,
        Command::List => list_command(bot, msg).await?,
        Command::Erase => erase_command(bot, msg).await?,
        Command::Help => help_command(bot, msg).await?,
        Command::Credits => credits_command(bot, msg).await?,
    }

    Ok(())
}

async fn start_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let username = match msg.chat.username() {
        Some(username) => username,
        None => "Unknown User",
    };

    let template = Templates::StartPage(username.to_string());

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn list_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        format!(
            r"<b>List Videonotes</b>

<i>work in progress</i>"
        ),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}

async fn erase_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::EraseConfirmationPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn help_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::HelpPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn credits_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::CreditsPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .disable_web_page_preview(true)
        .await?;

    Ok(())
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

    // Ok(())
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
    // Command::repl(bot, handle_commands).await;

    // Command::repl(bot, handle_commands).await;

    //     teloxide::repl(cloned_bot, |bot: Bot, msg: Message| async move {
    //         //
    //         log::info!("tanque chat_id: {:?}", msg.chat.id);
    //         let id = msg.chat.id;
    //         bot.send_message(msg.chat.id, "shit... I'm still alive...")
    //             .await?;
    //         Ok(())
    //     })
    //     .await;

    // let listener = teloxide::repl(bot, handle_input);
    // tokio::spawn(listener)
    Ok(())
}
