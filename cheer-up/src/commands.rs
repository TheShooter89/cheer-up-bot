use std::io::Error;

use log::debug;
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
    user::{get_user_by_id, UserId},
    utils::{get_user_folder_path, get_user_folder_path_by_user},
    videonotes::{delete_all_user_vnotes, get_random_vnote, get_vnote_list_from_db},
};

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "CheerUp Bot starting page")]
    Start,
    #[command(description = "Get random note from your friends")]
    RandomNote,
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
            "/start" => Some(Command::RandomNote),
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
        Command::RandomNote => random_note_command(&bot, msg).await?,
        Command::Extra => extra_command(&bot, msg).await?,
        Command::List => list_command(&bot, msg).await?,
        Command::Help => help_command(&bot, msg).await?,
        Command::Credits => credits_command(&bot, msg).await?,
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

pub async fn random_note_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .await?;

    let random_note = get_random_vnote(bot, &msg.chat).await?;
    let user = get_user_by_id(&random_note.user_id).await?;

    let mut user_folder = get_user_folder_path_by_user(&user);

    // this is needed because first 2 users in database
    // are fake for boilerplate on start up
    if random_note.user_id == 1 || random_note.user_id == 2 {
        user_folder = format!(
            "../_common_data/videonotes/{}_{}",
            user.telegram_id, user.username
        );
    }
    debug!("user_folder after user check is: {}", user_folder);

    let file_path = format!("{}/{}", user_folder, random_note.file_name);
    debug!("file_path is: {}", file_path);
    bot.send_video_note(msg.chat.id, InputFile::file(file_path))
        .await?;

    let template = Templates::RandomNotePage(user.username);

    let keyboard = keyboards::random_note_page(None);

    // bot.send_message(msg.chat.id, template.render())
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

pub async fn list_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let vnote_list = get_vnote_list_from_db(&msg.chat).await?;
    debug!("vnote_list is: {:?}", vnote_list);

    for vnote in &vnote_list {
        let user = get_user_by_id(&vnote.user_id).await?;

        let mut user_folder = get_user_folder_path_by_user(&user);

        // this is needed because first 2 users in database
        // are fake for boilerplate on start up
        if vnote.user_id == 1 || vnote.user_id == 2 {
            user_folder = format!(
                "../_common_data/videonotes/{}_{}",
                user.telegram_id, user.username
            );
        }
        debug!("user_folder after user check is: {}", user_folder);

        let file_path = format!("{}/{}", user_folder, vnote.file_name);
        debug!("file_path is: {}", file_path);
        bot.send_video_note(msg.chat.id, InputFile::file(file_path))
            .await?;
    }

    let template = Templates::ListPage(vnote_list.len().to_string());

    let keyboard = keyboards::list_notes_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn help_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::HelpPage;

    let keyboard = keyboards::help_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn credits_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let template = Templates::CreditsPage;

    let keyboard = keyboards::credits_page(None, None);

    bot.send_message(msg.chat.id, template.render())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}
