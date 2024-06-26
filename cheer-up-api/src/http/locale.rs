use axum::{
    extract::{Path, State},
    routing::{get, post},
    Extension, Json, Router,
};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePoolOptions, SqlitePool};
use std::{fmt, str::FromStr};

use crate::http::error::Error;
use crate::http::http::Result;

#[derive(Debug, Serialize, Deserialize)]
struct LocaleBody<T> {
    locale: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, FromRow)]
struct UserLocale {
    language: String,
}

impl fmt::Display for UserLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.language.as_str() {
            "en" => write!(f, "en"),
            "es" => write!(f, "es"),
            "it" => write!(f, "it"),
            "ua" => write!(f, "ua"),
            _ => write!(f, "en"),
        }
    }
}
impl UserLocale {
    pub fn locale(&self) -> Locale {
        match self.language.as_str() {
            "en" => Locale::EN,
            "es" => Locale::ES,
            "it" => Locale::IT,
            "ua" => Locale::UA,
            _ => Locale::EN,
        }
    }
}

pub fn router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route(
            "/api/locale/:user_id",
            get(get_user_locale).patch(set_user_locale),
        )
        .with_state(pool)
}

async fn get_user_locale(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<LocaleBody<Locale>>> {
    let user_locale: Option<UserLocale> = sqlx::query_as!(
        UserLocale,
        r#"
SELECT l.language as language
FROM users u
INNER JOIN locales l ON u.locale = l.id
WHERE u.id = ?
    "#,
        user_id
    )
    .fetch_optional(&pool)
    .await?;
    debug!("user_locale: {:?}", user_locale);

    let locale = user_locale.unwrap_or(UserLocale {
        language: "en".to_string(),
    });
    debug!("locale: {:?}", locale);

    Ok(Json(LocaleBody {
        locale: Locale::from_str(&locale.to_string()),
    }))
}

async fn set_user_locale(
    Path(user_id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(body): Json<LocaleBody<Locale>>,
) -> Result<Json<LocaleBody<Locale>>> {
    let _ = sqlx::query!(
        r#"
UPDATE users
SET locale = (SELECT id FROM locales WHERE language = ?)
WHERE id = ?
    "#,
        body.locale,
        user_id,
    )
    .execute(&pool)
    .await?;

    Ok(Json(LocaleBody {
        locale: body.locale,
    }))
}
