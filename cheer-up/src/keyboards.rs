use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::buttons::make_button;
use crate::callbacks::QueryData;

pub fn start_page(
    ask_friend_query: &QueryData,
    go_extra_query: &QueryData,
) -> InlineKeyboardMarkup {
    let ask_friend_button = make_button(&t!("start_page.buttons.ask_friend"), ask_friend_query);
    let go_to_extra_button = make_button(&t!("start_page.buttons.go_extra"), go_extra_query);

    let keyboard_buttons = vec![vec![ask_friend_button, go_to_extra_button]];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
