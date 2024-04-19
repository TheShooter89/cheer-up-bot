use std::io::Error;

use log::{debug, info};
use rust_i18n::set_locale;
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
    keyboards::{self, upload_page},
    locale::{get_user_locale_by_user_id, set_user_locale_by_user_id, Locale},
    stats::get_stats,
    templates::Templates,
    user::{get_user, get_user_by_id, get_user_by_telegram_id, save_user_to_db, UserId},
    utils::{get_user_folder_path, get_user_folder_path_by_user},
    videonotes::{
        delete_all_user_vnotes, delete_vnote_from_db, get_random_vnote, get_vnote_list_from_db,
    },
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
    #[command(description = "Upload a new video note")]
    Upload,
    #[command(description = "List all uploaded video notes")]
    List,
    #[command(description = "Change bot language")]
    Language,
    #[command(description = "Show help and available commands")]
    Help,
    #[command(description = "Show credits and code repo links")]
    Credits,
}

impl Command {
    pub fn parse_str(cmd: &str) -> Option<Command> {
        match cmd {
            "/start" => Some(Command::Start),
            "/ask_friend" => Some(Command::RandomNote),
            "/extra" => Some(Command::Extra),
            "/upload" => Some(Command::Upload),
            "/list" => Some(Command::List),
            "/language" => Some(Command::Language),
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
        Command::Upload => upload_command(&bot, msg).await?,
        Command::List => list_command(&bot, msg).await?,
        Command::Language => language_command(&bot, msg).await?,
        Command::Help => help_command(&bot, msg).await?,
        Command::Credits => credits_command(&bot, msg).await?,
    }

    Ok(())
}

pub async fn start_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[START_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[START_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let username = msg.chat.username().unwrap_or("Unknown User");

    let template = Templates::StartPage(username.to_string());

    let keyboard = keyboards::start_page(None, None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn random_note_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[RANDOM_NOTE_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!(
        "[RANDOM_NOTE_COMMAND] remote_locale is: {:?}",
        remote_locale
    );

    let locale_str = remote_locale.to_string();

    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
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

    let keyboard = keyboards::random_note_page(None, &remote_locale);

    // bot.send_message(msg.chat.id, template.render())
    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn delete_note_command(
    bot: &Bot,
    msg: Message,
    query_data: Option<Payload>,
) -> ResponseResult<()> {
    let data = query_data.unwrap_or(Payload::Text("none".to_string()));
    info!("[DELETE_NOTE_COMMAND] data is: {:?}", data);

    let user = get_user(&msg.chat).await?;
    info!("[DELETE_NOTE_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!(
        "[DELETE_NOTE_COMMAND] remote_locale is: {:?}",
        remote_locale
    );

    let locale_str = remote_locale.to_string();

    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .await?;

    info!("[DELETE_NOTE_COMMAND] deleting vnote with id: {:?}", &data);

    let template = Templates::DeleteNotePage(data.to_string());

    let keyboard = keyboards::delete_note_page(Some(data), &remote_locale);

    // bot.send_message(msg.chat.id, template.render())
    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn confirm_delete_command(
    bot: &Bot,
    msg: Message,
    query_data: Option<Payload>,
) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[CONFIRM_DELETE_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!(
        "[CONFIRM_DELETE_COMMAND] remote_locale is: {:?}",
        remote_locale
    );

    let locale_str = remote_locale.to_string();

    if query_data.is_none() {
        let keyboard = keyboards::delete_note_result_page(&remote_locale);
        bot.send_message(
            msg.chat.id,
            Templates::ErrorDeleteNotePage.render(&locale_str),
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
        return Ok(());
    }

    let data = query_data.unwrap();

    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .await?;

    info!("[CONFIRM_DELETE_COMMAND] deleting vnote with id: {}", &data);

    let parsed_data = data.to_string().parse::<i64>();
    if parsed_data.is_err() {
        let keyboard = keyboards::delete_note_result_page(&remote_locale);
        bot.send_message(
            msg.chat.id,
            Templates::ErrorDeleteNotePage.render(&locale_str),
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
        return Ok(());
    }
    let vnote_id = parsed_data.unwrap();
    // let deleted_note = delete_vnote_from_db(&vnote_id).await?;
    debug!("vnote_id is : {}", vnote_id);
    let deleted_note = delete_vnote_from_db(&vnote_id).await;

    if deleted_note.is_err() {
        info!(
            "[CONFIRM_DELETE_COMMAND] error deleting vnote with id: {}",
            &vnote_id
        );
        let keyboard = keyboards::delete_note_result_page(&remote_locale);
        bot.send_message(
            msg.chat.id,
            Templates::ErrorDeleteNotePage.render(&locale_str),
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
        return Ok(());
    }

    let unwrapped_deleted_note = deleted_note.unwrap();

    let template = Templates::SuccessDeleteNotePage(unwrapped_deleted_note.note);

    let keyboard = keyboards::delete_note_result_page(&remote_locale);

    // bot.send_message(msg.chat.id, template.render())
    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn erase_all_notes_command(
    bot: &Bot,
    msg: Message,
    query_data: Option<Payload>,
) -> ResponseResult<()> {
    let data = query_data.unwrap_or(Payload::Text("none".to_string()));
    info!("[erase_all_notes_COMMAND] data is: {:?}", data);

    let user = get_user(&msg.chat).await?;
    info!("[erase_all_notes_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!(
        "[erase_all_notes_COMMAND] remote_locale is: {:?}",
        remote_locale
    );

    let locale_str = remote_locale.to_string();

    info!(
        "[erase_all_notes_COMMAND] deleting vnote with data: {:?}",
        &data
    );

    let template = Templates::EraseAllNotesPage(data.to_string());

    let keyboard = keyboards::erase_all_notes_page(Some(data), &remote_locale);

    // bot.send_message(msg.chat.id, template.render())
    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn confirm_erase_all_notes_command(
    bot: &Bot,
    msg: Message,
    query_data: Option<Payload>,
) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[CONFIRM_DELETE_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!(
        "[CONFIRM_DELETE_COMMAND] remote_locale is: {:?}",
        remote_locale
    );

    let locale_str = remote_locale.to_string();

    if query_data.is_none() {
        let keyboard = keyboards::erase_all_notes_result_page(&remote_locale);
        bot.send_message(
            msg.chat.id,
            Templates::ErrorEraseAllNotesPage.render(&locale_str),
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
        return Ok(());
    }

    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .await?;

    let data = query_data.unwrap();

    info!("[CONFIRM_DELETE_COMMAND] query_data payload is: {}", &data);

    let parsed_data = data.number();
    if parsed_data.is_none() {
        let keyboard = keyboards::erase_all_notes_result_page(&remote_locale);
        bot.send_message(
            msg.chat.id,
            Templates::ErrorEraseAllNotesPage.render(&locale_str),
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
        return Ok(());
    }
    let vnote_id = parsed_data.unwrap();
    // let deleted_note = delete_vnote_from_db(&vnote_id).await?;
    info!("vnote_id is : {}", vnote_id);

    Ok(())
}

pub async fn extra_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[EXTRA_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[EXTRA_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let vnote_list = get_vnote_list_from_db(&msg.chat).await?;
    println!("vnote_list is: {:?}", vnote_list);

    let stats = get_stats().await?;

    let template = Templates::ExtraPage(
        msg.chat.username().unwrap_or("Unknown user").to_string(),
        // "42".to_string(),
        stats.total_videonotes.to_string(),
        stats.users.len().to_string(),
        stats.users,
    );

    let keyboard = keyboards::extra_page(None, None, None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn upload_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[UPLOAD_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[UPLOAD_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let vnote_list = get_vnote_list_from_db(&msg.chat).await?;
    println!("vnote_list is: {:?}", vnote_list);

    let stats = get_stats().await?;

    let template = Templates::UploadPage(
        msg.chat.username().unwrap_or("Unknown user").to_string(),
        // "42".to_string(),
        stats.total_videonotes.to_string(),
        stats.users.len().to_string(),
        stats.users,
    );

    let keyboard = upload_page(None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn list_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[LIST_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[LIST_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    // show loading indicator
    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .await?;

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

        let keyboard = keyboards::vnote_entry(Some(Payload::NoteId(vnote.id)), &remote_locale);
        bot.send_video_note(msg.chat.id, InputFile::file(file_path))
            .reply_markup(keyboard)
            .await?;
    }

    let template = Templates::ListPage(user.username, vnote_list.len().to_string());

    let keyboard = keyboards::list_notes_page(Some(Payload::UserId(user.id)), None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn language_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[LANGUAGE_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[LANGUAGE_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let template = Templates::LanguagePage;

    let keyboard = keyboards::language_page(&remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .disable_web_page_preview(true)
        .await?;
    Ok(())
}

pub async fn help_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[HELP_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[HELP_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let template = Templates::HelpPage;

    let keyboard = keyboards::help_page(None, None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn credits_command(bot: &Bot, msg: Message) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[CREDITS_COMMAND] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[CREDITS_COMMAND] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    let template = Templates::CreditsPage;

    let keyboard = keyboards::credits_page(None, None, &remote_locale);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}

pub async fn set_language_command(bot: &Bot, msg: Message, locale: Locale) -> ResponseResult<()> {
    let user = get_user_by_telegram_id(&msg.chat).await?;
    let user_locale = set_user_locale_by_user_id(&user.id, &locale).await?;
    let remote_locale = get_user_locale_by_user_id(&user.id).await?;

    let locale_str = remote_locale.to_string();

    info!("setting locale to: {:?}", user_locale);
    // set_locale(&locale_str);

    let username = msg.chat.username().unwrap_or("Unknown User");

    let template = Templates::StartPage(username.to_string());

    let keyboard = keyboards::start_page(None, None, &remote_locale);

    let rendered_template = template.render(&locale_str);
    debug!("rendered_template: {:?}", rendered_template);

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}
