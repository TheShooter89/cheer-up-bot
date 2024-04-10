use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::buttons::{
    ask_friend_button, go_to_credits_button, go_to_extra_button, go_to_help_button,
    go_to_home_button, go_to_language_button, list_all_notes_button, make_button,
    set_language_EN_button, set_language_ES_button, set_language_IT_button, set_language_UA_button,
};
use crate::callbacks::{Payload, QueryData};

pub fn start_page(
    ask_friend_query: Option<Payload>,
    go_extra_query: Option<Payload>,
) -> InlineKeyboardMarkup {
    let ask_friend_button = ask_friend_button(ask_friend_query.clone());
    // let go_to_extra_button = make_button(&t!("start_page.buttons.go_extra"), go_extra_query);
    let go_to_extra_button = go_to_extra_button(go_extra_query.clone());

    let keyboard_buttons = vec![vec![ask_friend_button, go_to_extra_button]];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn random_note_page(ask_friend_payload: Option<Payload>) -> InlineKeyboardMarkup {
    let row_1 = vec![ask_friend_button(ask_friend_payload.clone())];

    let row_2 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn list_notes_page(
    go_extra_payload: Option<Payload>,
    go_help_payload: Option<Payload>,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_extra_button(go_extra_payload.clone()),
        go_to_help_button(go_help_payload.clone()),
    ];

    let row_2 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn extra_page(
    list_all_query_payload: Option<Payload>,
    go_credits_payload: Option<Payload>,
    go_language_payload: Option<Payload>,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_language_button(go_language_payload.clone()),
        go_to_credits_button(go_credits_payload.clone()),
    ];

    let row_2 = vec![list_all_notes_button(list_all_query_payload.clone())];

    let row_3 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2, row_3];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn credits_page(
    go_extra_payload: Option<Payload>,
    go_help_payload: Option<Payload>,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_extra_button(go_extra_payload.clone()),
        go_to_help_button(go_help_payload.clone()),
    ];

    let row_2 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn language_page() -> InlineKeyboardMarkup {
    let row_1 = vec![set_language_EN_button(Some(Payload::Text(
        "en".to_string(),
    )))];

    let row_2 = vec![set_language_ES_button(Some(Payload::Text(
        "es".to_string(),
    )))];

    let row_3 = vec![set_language_IT_button(Some(Payload::Text(
        "it".to_string(),
    )))];

    let row_4 = vec![set_language_UA_button(Some(Payload::Text(
        "ua".to_string(),
    )))];

    let row_5 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2, row_3, row_4, row_5];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn help_page(
    go_extra_payload: Option<Payload>,
    go_credits_payload: Option<Payload>,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_language_button(go_extra_payload.clone()),
        go_to_credits_button(go_credits_payload.clone()),
    ];

    let row_2 = vec![go_to_home_button(None)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
