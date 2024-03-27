use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::callbacks::QueryData;

pub fn start_page(
    ask_friend_query: &QueryData,
    go_extra_query: &QueryData,
) -> InlineKeyboardMarkup {
    let ask_friend_button_data =
        serde_json::to_string(&ask_friend_query).unwrap_or("none".to_string());
    let ask_friend_button = InlineKeyboardButton::new(
        t!("start_page.buttons.ask_friend"),
        InlineKeyboardButtonKind::CallbackData(ask_friend_button_data),
    );

    let go_to_extra_button_data =
        serde_json::to_string(&go_extra_query).unwrap_or("none".to_string());
    let go_to_extra_button = InlineKeyboardButton::new(
        t!("start_page.buttons.go_extra"),
        InlineKeyboardButtonKind::CallbackData(go_to_extra_button_data),
    );

    let keyboard_buttons = vec![vec![ask_friend_button, go_to_extra_button]];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
