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

    make_button(&t!("start_page.buttons.ask_friend"), &query_data)
}
