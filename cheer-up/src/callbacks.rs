use teloxide::{
    prelude::*,
    types::{CallbackQuery, Chat},
    Bot, RequestError,
};

use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::user::UserId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryData {
    pub topic: Topic,
    pub payload: Option<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Payload {
    Text(String),
    Username(String),
    UserId(i64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Topic {
    RandomNote,
    ExtraPage,
    ShowId,
    Unknown,
}

impl Topic {
    pub fn name(&self) -> String {
        match self {
            Topic::RandomNote => "#random_note".to_string(),
            Topic::ExtraPage => "#extra".to_string(),
            Topic::ShowId => "#show_id".to_string(),
            Topic::Unknown => "none".to_string(),
        }
    }
}

pub async fn handle_callback(bot: Bot, query: CallbackQuery) -> Result<(), RequestError> {
    // suppress waiting spinner from button after user taps it
    bot.answer_callback_query(query.id).await?;

    let chat = match query.message {
        Some(msg) => Some(msg.chat),
        None => None,
    };

    let raw_data = match query.data {
        Some(data) => data,
        None => "none".to_string(),
    };
    // println!("callback query data is: {:#?}", raw_data);

    let callback_query_data = serde_json::from_str::<QueryData>(&raw_data);
    // println!("callback query data is: {:#?}", callback_query_data);

    match callback_query_data {
        Ok(data) => {
            let topic = data.topic;

            // INFO: call handlers based on query topic passing optional payload
            match topic {
                Topic::RandomNote => handle_random_note(&bot, chat, data.payload).await?,
                _ => println!("other"),
            }

            Ok(())
        }
        Err(e) => {
            println!("ERROR WHILE PARSING CALLBACK DATA: {:?}", e);
            Ok(())
        }
    }
}

async fn handle_random_note(
    bot: &Bot,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => match data {
                    Payload::Text(text) => {
                        bot.send_message(
                            chat.id,
                            format!(
                                "you requested a random videonote, and your payload is: {}",
                                text
                            ),
                        )
                        .await?;
                        Ok(())
                    }
                    // RandomNote callback needs Payload::Text only
                    _ => Ok(()),
                },
                None => {
                    // no Payload provided
                    bot.send_message(
                        chat.id,
                        "you requested a random videonote without a payload",
                    )
                    .await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => Ok(()),
    }
}
