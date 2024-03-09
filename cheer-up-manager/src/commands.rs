use std::io::Error;

use teloxide::{
    payloads::SendMessageSetters, prelude::*, types::ParseMode, utils::command::BotCommands,
};
use tokio::fs;

use crate::templates::Templates;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
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

pub async fn handle_commands(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Start => start_command(bot, msg).await?,
        Command::List => list_command(bot, msg).await?,
        Command::Erase => erase_command(bot, msg).await?,
        Command::Help => help_command(bot, msg).await?,
        Command::Credits => credits_command(bot, msg).await?,
    }

    Ok(())
}

pub async fn start_command(bot: Bot, msg: Message) -> ResponseResult<()> {
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
