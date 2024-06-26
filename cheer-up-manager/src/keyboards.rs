use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::buttons::{
    confirm_delete_button, confirm_erase_all_notes_button, delete_note_button,
    erase_all_notes_button, go_to_credits_button, go_to_help_button, go_to_home_button,
    go_to_language_button, go_to_upload_button, list_all_notes_button, set_language_EN_button,
    set_language_ES_button, set_language_IT_button, set_language_UA_button,
};
use crate::callbacks::{Payload, QueryData};
use crate::locale::Locale;

pub fn start_page(locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![
        list_all_notes_button(None, locale),
        go_to_help_button(None, locale),
    ];

    let row_2 = vec![go_to_upload_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn list_notes_page(
    erase_all_notes_payload: Option<Payload>,
    go_help_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        erase_all_notes_button(erase_all_notes_payload.clone(), locale),
        go_to_help_button(go_help_payload.clone(), locale),
    ];

    let row_2 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn erase_all_notes_page(
    confirm_delete_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        confirm_erase_all_notes_button(confirm_delete_payload.clone(), locale),
        go_to_home_button(None, locale),
    ];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn erase_all_notes_result_page(locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn vnote_entry(delete_note_payload: Option<Payload>, locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![delete_note_button(delete_note_payload.clone(), locale)];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn delete_note_page(
    confirm_delete_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        confirm_delete_button(confirm_delete_payload.clone(), locale),
        go_to_home_button(None, locale),
    ];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn delete_note_result_page(locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn upload_page(
    list_all_query_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        list_all_notes_button(list_all_query_payload.clone(), locale),
        go_to_help_button(None, locale),
    ];

    let row_2 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn upload_result_page(locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn credits_page(
    go_language_payload: Option<Payload>,
    go_help_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_language_button(go_language_payload.clone(), locale),
        go_to_help_button(go_help_payload.clone(), locale),
    ];

    let row_2 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn language_page(locale: &Locale) -> InlineKeyboardMarkup {
    let row_1 = vec![set_language_EN_button(
        Some(Payload::Text("en".to_string())),
        locale,
    )];

    let row_2 = vec![set_language_ES_button(
        Some(Payload::Text("es".to_string())),
        locale,
    )];

    let row_3 = vec![set_language_IT_button(
        Some(Payload::Text("it".to_string())),
        locale,
    )];

    let row_4 = vec![set_language_UA_button(
        Some(Payload::Text("ua".to_string())),
        locale,
    )];

    let row_5 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2, row_3, row_4, row_5];

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn help_page(
    go_language_payload: Option<Payload>,
    go_credits_payload: Option<Payload>,
    locale: &Locale,
) -> InlineKeyboardMarkup {
    let row_1 = vec![
        go_to_language_button(go_language_payload.clone(), locale),
        go_to_credits_button(go_credits_payload.clone(), locale),
    ];

    let row_2 = vec![go_to_home_button(None, locale)];

    let keyboard_buttons = vec![row_1, row_2];

    InlineKeyboardMarkup::new(keyboard_buttons)
}
