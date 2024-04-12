use teloxide::{
    prelude::*,
    types::{Me, ParseMode},
};

use crate::templates::*;
use crate::user::*;
use crate::videonotes::*;
use crate::{commands::*, locale::get_user_locale_by_user_id};

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
    let message_type = MessageType::from_msg(&msg);
    println!("Message type you sent: {:?}", message_type);

    let chat_id = msg.chat.id;

    match message_type {
        MessageType::VideoNote => {
            println!("received video note");
            let vnote = msg.video_note().unwrap();

            let save_user = save_user_to_db(&msg.chat).await;
            if save_user.is_err() {
                println!("an error occurred while saving user to db")
            }
            upload_vnote(&bot, vnote, &msg.chat).await?;
            let save_vnote = save_vnote_to_db(vnote, &msg.chat).await;
            if save_vnote.is_err() {
                println!("an error occurred while saving note");
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
