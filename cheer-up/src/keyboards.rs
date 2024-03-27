use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::callbacks::QueryData;

pub fn start_page(query: &QueryData) -> InlineKeyboardMarkup {
    let button_text = t!("start_page.button");

    let serialized_callback_data = serde_json::to_string(&query).unwrap_or("none".to_string());

    let keyboard_buttons = vec![vec![InlineKeyboardButton::new(
        button_text,
        InlineKeyboardButtonKind::CallbackData(serialized_callback_data),
    )]];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
