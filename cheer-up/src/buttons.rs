use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::callbacks::QueryData;

pub fn make_button(label: &str, query_data: &QueryData) -> InlineKeyboardButton {
    let data = serde_json::to_string(query_data).unwrap_or("none".to_string());

    InlineKeyboardButton::new(label, InlineKeyboardButtonKind::CallbackData(data))
}
