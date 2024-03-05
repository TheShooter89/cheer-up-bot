use std::io::Error;

use log;
use teloxide::{net::Download, prelude::*, types::InputFile, RequestError};
use tokio::fs;

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

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        log::info!("Throwing dice bot...");
        println!("Throwing dice bot...");
        println!("msg.video_note(): {:?}", msg.video_note());
        match msg.video_note() {
            Some(vnote) => {
                //
                let vnote_file_id = vnote.file.id.clone();
                println!("vnote: {:?}", vnote);
                println!("vnote.id: {:?}", vnote.file.id);

                // bot.send_video_note(msg.chat.id, InputFile::file_id(vnote_file_id.clone()))
                //     .await?;

                println!("vnote sent to {:?}", msg.chat.id.to_string());

                match download_vnote(&bot, &vnote_file_id, msg.chat.id).await {
                    Ok(res) => println!("video note succefully downloaded, result: {:?}", res),
                    Err(e) => println!("error while downloading video note, error: {:?}", e),
                };
                bot.send_message(msg.chat.id, "Specchio riflesso, come alle elementari LOL")
                    .await
                    .unwrap();
                ()
            }
            None => println!("msg is not a video note"),
        }
        // bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
