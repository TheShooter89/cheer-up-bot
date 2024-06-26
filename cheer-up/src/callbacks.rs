use log::*;
use std::fmt;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, Chat},
    Bot, RequestError,
};

use serde::{self, Deserialize, Serialize};
use serde_json;

use crate::{commands, locale::Locale, user::UserId};

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
    NoteId(i64),
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Payload::Text(text) => write!(f, "{}", text),
            Payload::Username(text) => write!(f, "{}", text),
            Payload::UserId(id) => write!(f, "{}", id),
            Payload::NoteId(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Topic {
    GetRandomNote,
    ListAllNotes,
    DeleteNote,
    ConfirmDelete,
    GoHomePage,
    GoExtraPage,
    GoUploadPage,
    GoCreditsPage,
    GoLanguagePage,
    GoHelpPage,
    SetLanguage,
}

impl Topic {
    pub fn name(&self) -> String {
        match self {
            Topic::GetRandomNote => "#random_note".to_string(),
            Topic::ListAllNotes => "#list".to_string(),
            Topic::DeleteNote => "#delete".to_string(),
            Topic::ConfirmDelete => "#confirm_delete".to_string(),
            Topic::GoHomePage => "#home".to_string(),
            Topic::GoExtraPage => "#extra".to_string(),
            Topic::GoUploadPage => "#upload".to_string(),
            Topic::GoCreditsPage => "#credits".to_string(),
            Topic::GoLanguagePage => "#language".to_string(),
            Topic::GoHelpPage => "#help".to_string(),
            Topic::SetLanguage => "#set_language".to_string(),
        }
    }
}

pub async fn handle_callback(bot: Bot, query: CallbackQuery) -> Result<(), RequestError> {
    // suppress waiting spinner from button after user taps it
    bot.answer_callback_query(query.id).await?;

    let message = query.message.clone();
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
                Topic::GetRandomNote => {
                    handle_random_note(&bot, message, chat, data.payload).await?
                }
                Topic::DeleteNote => handle_delete_note(&bot, message, chat, data.payload).await?,
                Topic::ConfirmDelete => {
                    handle_confirm_delete(&bot, message, chat, data.payload).await?
                }
                Topic::ListAllNotes => {
                    handle_list_all_notes(&bot, message, chat, data.payload).await?
                }
                Topic::GoHomePage => handle_go_home_page(&bot, message, chat, data.payload).await?,
                Topic::GoExtraPage => {
                    handle_go_extra_page(&bot, message, chat, data.payload).await?
                }
                Topic::GoUploadPage => {
                    handle_go_upload_page(&bot, message, chat, data.payload).await?
                }
                Topic::GoCreditsPage => {
                    handle_go_credits_page(&bot, message, chat, data.payload).await?
                }
                Topic::GoLanguagePage => {
                    handle_go_language_page(&bot, message, chat, data.payload).await?
                }
                Topic::GoHelpPage => handle_go_help_page(&bot, message, chat, data.payload).await?,
                Topic::SetLanguage => {
                    handle_set_language(&bot, message, chat, data.payload).await?
                }
                _ => warn!("unkwnown topic"),
            }

            Ok(())
        }
        Err(e) => {
            println!("ERROR WHILE PARSING CALLBACK DATA: {:?}", e);
            Ok(())
        }
    }
}

// INFO: all combination of payloads are handled,
//      even if not necessary for this specific handler,
//      as an explainatory example
async fn handle_random_note(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => match data {
                    Payload::Text(_text) => {
                        // bot.send_message(
                        //     chat.id,
                        //     format!(
                        //         "you requested a random videonote, and your payload is: {}",
                        //         text
                        //     ),
                        // )
                        // .await?;

                        // INFO: we can safely unwrap msg, since chat is extracted from query.message
                        // iteslf
                        commands::random_note_command(bot, msg.unwrap()).await?;
                        Ok(())
                    }
                    // RandomNote callback needs Payload::Text only
                    _ => {
                        warn!("payload provided is not Payload::Text");
                        commands::random_note_command(bot, msg.unwrap()).await?;
                        Ok(())
                    }
                },
                None => {
                    // no Payload provided
                    warn!("no Payload provided");
                    commands::random_note_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_delete_note(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    commands::delete_note_command(bot, msg.unwrap(), Some(data)).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::delete_note_command(bot, msg.unwrap(), None).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_confirm_delete(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    info!("Payload data is: {:?}", data);
                    commands::confirm_delete_command(bot, msg.unwrap(), Some(data)).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::confirm_delete_command(bot, msg.unwrap(), None).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_list_all_notes(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    commands::list_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::list_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_home_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    commands::start_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::start_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_extra_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::extra_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_upload_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::upload_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_credits_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::credits_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_language_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    commands::language_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::language_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_go_help_page(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    warn!("Payload provided, but not needed");
                    commands::help_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::help_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}

async fn handle_set_language(
    bot: &Bot,
    msg: Option<teloxide::types::Message>,
    target: Option<Chat>,
    payload: Option<Payload>,
) -> ResponseResult<()> {
    match target {
        Some(chat) => {
            match payload {
                Some(data) => {
                    info!("button data is: {:#?}", data);
                    let selected_locale = Locale::from_str(data.to_string().as_str());
                    info!("selected locale is: {:?}", selected_locale);
                    commands::set_language_command(bot, msg.unwrap(), selected_locale).await?;
                    Ok(())
                }
                None => {
                    // no Payload provided
                    commands::help_command(bot, msg.unwrap()).await?;
                    Ok(())
                }
            }
        }
        // No target Chat available
        None => {
            warn!("target Chat is None");
            Ok(())
        }
    }
}
