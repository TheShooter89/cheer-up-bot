use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::{requests::ResponseResult, types::Chat};

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
struct LocaleBody<T> {
    locale: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "en")]
    EN,
    #[serde(rename = "es")]
    ES,
    #[serde(rename = "it")]
    IT,
    #[serde(rename = "ua")]
    UA,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Locale::EN => write!(f, "en"),
            Locale::ES => write!(f, "es"),
            Locale::IT => write!(f, "it"),
            Locale::UA => write!(f, "ua"),
        }
    }
}

impl Locale {
    pub fn from_str(locale: &str) -> Locale {
        match locale {
            "en" => Locale::EN,
            "es" => Locale::ES,
            "it" => Locale::IT,
            "ua" => Locale::UA,
            _ => Locale::EN,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct UserLocale {
    language: String,
}

impl fmt::Display for UserLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl UserLocale {
    pub fn locale(&self) -> Locale {
        match self.language.as_str() {
            "en" => Locale::EN,
            "it" => Locale::IT,
            "ua" => Locale::UA,
            _ => Locale::EN,
        }
    }
}

pub async fn get_user_locale_by_user_id(user_id: &i64) -> ResponseResult<Locale> {
    let client = Client::new();

    let resp = client
        .get(format!("http://0.0.0.0:1989/api/locale/{}", user_id))
        .send()
        .await?
        .json::<LocaleBody<Locale>>()
        .await?;

    println!("user {:#?} locale is: {:#?}", user_id, resp);

    Ok(resp.locale)
}

pub async fn set_user_locale_by_user_id(user_id: &i64, locale: &Locale) -> ResponseResult<Locale> {
    let new_locale = LocaleBody {
        locale: locale.clone(),
    };

    let client = Client::new();

    let resp = client
        .patch(format!("http://0.0.0.0:1989/api/locale/{}", user_id))
        .json::<LocaleBody<Locale>>(&new_locale)
        .send()
        .await?
        .json::<LocaleBody<Locale>>()
        .await?;

    println!("user {:#?} locale is: {:#?}", user_id, resp);

    Ok(resp.locale)
}
