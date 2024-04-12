use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::{
    callbacks::{Payload, QueryData, Topic},
    locale::Locale,
};

pub fn make_button(label: &str, query_data: &QueryData) -> InlineKeyboardButton {
    let data = serde_json::to_string(query_data).unwrap_or("none".to_string());

    InlineKeyboardButton::new(label, InlineKeyboardButtonKind::CallbackData(data))
}

pub fn ask_friend_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GetRandomNote,
        payload,
    };

    make_button(
        &t!("buttons.ask_friend", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn list_all_notes_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::ListAllNotes,
        payload,
    };

    make_button(
        &t!("buttons.list_notes", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn go_to_home_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoHomePage,
        payload,
    };

    make_button(
        &t!("buttons.go_home", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn go_to_extra_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoExtraPage,
        payload,
    };

    make_button(
        &t!("buttons.go_extra", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn go_to_credits_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoCreditsPage,
        payload,
    };

    make_button(
        &t!("buttons.go_credits", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn go_to_language_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoLanguagePage,
        payload,
    };

    make_button(
        &t!("buttons.go_language", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn go_to_help_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::GoHelpPage,
        payload,
    };

    make_button(
        &t!("buttons.go_help", locale = locale.to_string().as_str()),
        &query_data,
    )
}

pub fn set_language_EN_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::SetLanguage,
        payload,
    };

    make_button(
        &t!(
            "buttons.set_language_EN",
            locale = locale.to_string().as_str()
        ),
        &query_data,
    )
}

pub fn set_language_ES_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::SetLanguage,
        payload,
    };

    make_button(
        &t!(
            "buttons.set_language_ES",
            locale = locale.to_string().as_str()
        ),
        &query_data,
    )
}

pub fn set_language_UA_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::SetLanguage,
        payload,
    };

    make_button(
        &t!(
            "buttons.set_language_UA",
            locale = locale.to_string().as_str()
        ),
        &query_data,
    )
}

pub fn set_language_IT_button(payload: Option<Payload>, locale: &Locale) -> InlineKeyboardButton {
    let query_data = QueryData {
        topic: Topic::SetLanguage,
        payload,
    };

    make_button(
        &t!(
            "buttons.set_language_IT",
            locale = locale.to_string().as_str()
        ),
        &query_data,
    )
}
