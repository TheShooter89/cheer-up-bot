use std::io::Error;

use log;
use teloxide::{
    net::Download,
    payloads::SendMessageSetters,
    prelude::*,
    types::{InputFile, ParseMode},
    utils::command::BotCommands,
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
    pub fn parse_str(cmd: &str) -> Command {
        match cmd {
            "/start" => Command::Start,
            "/list" => Command::List,
            "/erase" => Command::Erase,
            "/help" => Command::List,
            "/credits" => Command::Credits,
            _ => Command::Start,
        }
    }
}

async fn handle_commands(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        println!("message received is text: {:?}", text);
        let parsed_cmd = Command::parse_str(&text);
        println!("message received is command: {:?}", parsed_cmd);
    }

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

    let parsed_cmd = Command::parse_str(msg.text().unwrap_or("none"));
    println!("Command you sent: {:?}", parsed_cmd);

    handle_commands(bot.clone(), parsed_cmd, msg.clone()).await;

    bot.send_message(
        msg.chat.id,
        format!("Message type you sent: {:?}", message_type),
    )
    .await?;
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
