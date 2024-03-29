use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::callbacks::{Payload, QueryData, Topic};

pub fn make_button(label: &str, query_data: &QueryData) -> InlineKeyboardButton {
    let data = serde_json::to_string(query_data).unwrap_or("none".to_string());

    InlineKeyboardButton::new(label, InlineKeyboardButtonKind::CallbackData(data))
}

pub fn ask_friend_button(payload: Option<Payload>) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GetRandomNote,
        payload,
    };

    make_button(&t!("buttons.ask_friend"), &query_data)
}

pub fn go_to_extra_button(payload: Option<Payload>) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoExtraPage,
        payload,
    };

    make_button(&t!("buttons.go_extra"), &query_data)
}

pub fn go_to_credits_button(payload: Option<Payload>) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoCreditsPage,
        payload,
    };

    make_button(&t!("buttons.go_cedits"), &query_data)
}

pub fn go_to_help_button(payload: Option<Payload>) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoHelpPage,
        payload,
    };

    make_button(&t!("buttons.go_help"), &query_data)
}
