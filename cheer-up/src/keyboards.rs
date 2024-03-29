use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::buttons::{
    ask_friend_button, go_to_credits_button, go_to_extra_button, go_to_home_button,
    list_all_notes_button, make_button,
};
use crate::callbacks::{Payload, QueryData};

pub fn start_page(
    ask_friend_query: &QueryData,
    go_extra_query: &QueryData,
) -> InlineKeyboardMarkup {
    let ask_friend_button = ask_friend_button(ask_friend_query.payload.clone());
    // let go_to_extra_button = make_button(&t!("start_page.buttons.go_extra"), go_extra_query);
    let go_to_extra_button = go_to_extra_button(go_extra_query.payload.clone());

    let keyboard_buttons = vec![vec![ask_friend_button, go_to_extra_button]];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn extra_page(
    list_all_query_payload: Option<Payload>,
    go_credits_query: Option<Payload>,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        list_all_notes_button(list_all_query_payload.clone()),
        go_to_credits_button(go_credits_query.clone()),
    ];

    let row_2 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
