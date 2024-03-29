use std::io::Error;

use serde_json::json;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InputFile, KeyboardButton, KeyboardMarkup,
        ParseMode,
    },
    utils::command::BotCommands,
};
use tokio::fs;

use crate::{
    callbacks::{Payload, QueryData, Topic},
    keyboards,
    templates::Templates,
    user::UserId,
    utils::get_user_folder_path,
    videonotes::{delete_all_user_vnotes, get_vnote_list_from_db},
};

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "CheerUp Bot starting page")]
    Start,
    #[command(description = "Show Extra page")]
    Extra,
    #[command(description = "List all uploaded video notes")]
    List,
    #[command(description = "Show help and available commands")]
    Help,
    #[command(description = "Show credits and code repo links")]
    Credits,
}

impl Command {
    pub fn parse_str(cmd: &str) -> Option<Command> {
        match cmd {
            "/start" => Some(Command::Start),
            "/extra" => Some(Command::Extra),
            "/list" => Some(Command::List),
            "/help" => Some(Command::Help),
            "/credits" => Some(Command::Credits),
            _ => None,
        }
    }
}

pub async fn handle_commands(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Start => start_command(&bot, msg).await?,
        Command::Extra => extra_command(&bot, msg).await?,
        Command::List => list_command(bot, msg).await?,
        Command::Help => help_command(bot, msg).await?,
        Command::Credits => credits_command(bot, msg).await?,
    }

    Ok(())
}

pub async fn start_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let username = msg.chat.username().unwrap_or("Unknown User");

    let template = Templates::StartPage(username.to_string());

    let keyboard = keyboards::start_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn extra_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let vnote_list = get_vnote_list_from_db(&msg.chat).await?;
    println!("vnote_list is: {:?}", vnote_list);

    let template = Templates::ExtraPage(
        "tanqueshen".to_string(),
        // "42".to_string(),
        vnote_list.len().to_string(),
        "3".to_string(),
        "@tanqueshen uploaded 9 notes\n@che uploaded 6 notes".to_string(),
    );

    let keyboard = keyboards::extra_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn list_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let vnote_list = get_vnote_list_from_db(&msg.chat).await?;
    println!("vnote_list is: {:?}", vnote_list);

    let user_folder = get_user_folder_path(&msg.chat);

    for vnote in &vnote_list {
        let file_path = format!("{}/{}", user_folder, vnote.file_name);
        bot.send_video_note(msg.chat.id, InputFile::file(file_path))
            .await?;
    }

    bot.send_message(
        msg.chat.id,
        format!(r"You uploaded <b>{}</b> video notes", vnote_list.len()),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}

pub async fn erase_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::EraseConfirmationPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn erase_confirmation_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    match delete_all_user_vnotes(&msg.chat).await {
        Ok(_) => {
            let template = Templates::EraseConfirmationCompletedPage;

            bot.send_message(msg.chat.id, template.render())
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Err(_) => {
            let template = Templates::EraseConfirmationErrorPage;

            bot.send_message(msg.chat.id, template.render())
                .parse_mode(ParseMode::Html)
                .await?;
        }
    };

    Ok(())
}

pub async fn help_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::HelpPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;

    let ask_friend_callback_data = QueryData {
        topic: Topic::GetRandomNote,
        // payload: Some(Payload::Text("prova".to_string())),
        payload: None,
    };

    let go_to_extra_callback_data = QueryData {
        topic: Topic::GoExtraPage,
        // payload: Some(Payload::Text("prova".to_string())),
        payload: None,
    };

    let keyboard = keyboards::start_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn credits_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::CreditsPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}
