use log::{debug, info};
use teloxide::{
    prelude::*,
    types::{Me, ParseMode},
};

use crate::user::*;
use crate::videonotes::*;
use crate::{commands::*, locale::get_user_locale_by_user_id};
use crate::{keyboards, templates::*};

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

pub async fn handle_message(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    let user = get_user(&msg.chat).await?;
    info!("[HANDLE_MESSAGE] user is: {:?}", user);

    let remote_locale = get_user_locale_by_user_id(&user.id).await?;
    info!("[HANDLE_MESSAGE] remote_locale is: {:?}", remote_locale);

    let locale_str = remote_locale.to_string();

    // INFO: show loading indicator
    let template = Templates::LoadingPage;

    bot.send_message(msg.chat.id, template.render(&locale_str))
        .parse_mode(ParseMode::Html)
        .await?;

    // INFO: parse message to intercept videonotes uploadings
    let message_type = MessageType::from_msg(&msg);
    debug!("[HANDLE_MESSAGE] Message type you sent: {:?}", message_type);

    let chat_id = msg.chat.id;

    match message_type {
        MessageType::VideoNote => {
            info!("[HANDLE_MESSAGE] received video note");
            let vnote = msg.video_note().unwrap();

            // INFO: save user to db
            let save_user = save_user_to_db(&msg.chat).await;
            if save_user.is_err() {
                info!("[HANDLE_MESSAGE] an error occurred while saving user to db");
            }

            // INFO: upload vnote to server disk
            let upload_result = upload_vnote(&bot, vnote, &msg.chat).await;
            if upload_result.is_err() {
                info!("[HANDLE_MESSAGE] an error occurred while saving note");
                let keyboard = keyboards::upload_result_page(&remote_locale);
                bot.send_message(msg.chat.id, Templates::ErrorUploadPage.render(&locale_str))
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
                return Ok(());
            }

            // INFO: save vnote to db
            let save_vnote = save_vnote_to_db(&vnote.file.id, &msg.chat).await;
            if save_vnote.is_err() {
                info!("[HANDLE_MESSAGE] an error occurred while saving note");
                let keyboard = keyboards::upload_result_page(&remote_locale);
                bot.send_message(msg.chat.id, Templates::ErrorUploadPage.render(&locale_str))
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
                return Ok(());
            }

            let template = Templates::SuccessUploadPage;
            let keyboard = keyboards::upload_result_page(&remote_locale);

            // bot.send_message(msg.chat.id, template.render())
            bot.send_message(msg.chat.id, template.render(&locale_str))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
            Ok(())
        }
        MessageType::Text => {
            let command = Command::parse_str(msg.text().unwrap_or("none"));

            info!(
                "[HANDLE_MESSAGE] Text you sent: {:?}",
                msg.text().unwrap_or("NONE")
            );

            if let Some(cmd) = command {
                info!("[HANDLE_MESSAGE] Command you sent: {:?}", cmd);
                handle_commands(bot.clone(), cmd, msg.clone()).await?;

                return Ok(());
            }

            start_command(&bot, msg.clone()).await?;

            Ok(())
        }
        _ => {
            let user = get_user_by_telegram_id(&msg.chat).await?;
            let remote_locale = get_user_locale_by_user_id(&user.id).await?;

            let locale_str = remote_locale.to_string();

            bot.send_message(
                chat_id,
                Templates::UnsupportedInputPage(message_type.name().to_string())
                    .render(&locale_str),
            )
            .parse_mode(ParseMode::Html)
            .await?;

            Ok(())
        }
    }
}
