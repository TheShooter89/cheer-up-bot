use dotenvy as dotenv;
use log::debug;

use crate::stats::UserStats;

#[derive(Debug, Clone)]
pub enum Templates {
    LoadingPage,
    StartPage(String),
    RandomNotePage(String),
    DeleteNotePage(String),
    SuccessDeleteNotePage(String),
    ErrorDeleteNotePage,
    UploadPage(String, String, String, Vec<UserStats>),
    SuccessUploadPage,
    ErrorUploadPage,
    ListPage(String, String),
    EraseAllNotesPage(String),
    SuccessEraseAllNotesPage(String),
    ErrorEraseAllNotesPage,
    LanguagePage,
    HelpPage,
    CreditsPage,
    UnsupportedInputPage(String),
}

impl Templates {
    pub fn render(&self, locale: &str) -> String {
        debug!("rendering with locale: {:?}", locale);
        let author =
            dotenv::var("AUTHOR").expect("error loading author from envirenment variables");

        let profile_name = dotenv::var("AUTHOR_PROFILE_NAME")
            .expect("error loading author from envirenment variables");

        let profile_url = dotenv::var("AUTHOR_PROFILE_URL")
            .expect("error loading author profile url from envirenment variables");

        let repo_url = dotenv::var("CODE_REPO_URL")
            .expect("error loading code repo url from envirenment variables");

        match self {
            Templates::LoadingPage => loading_page(locale),
            Templates::StartPage(user) => start_page(user, locale),
            Templates::RandomNotePage(user) => random_note_page(user, locale),
            Templates::DeleteNotePage(note_id) => delete_note_page(note_id, locale),
            Templates::SuccessDeleteNotePage(note_id) => success_delete_note_page(note_id, locale),
            Templates::ErrorDeleteNotePage => error_delete_note_page(locale),
            Templates::UploadPage(user, total_notes, total_users, user_videonotes_list) => {
                upload_page(user, total_notes, total_users, user_videonotes_list, locale)
            }
            Templates::SuccessUploadPage => success_upload_page(locale),
            Templates::ErrorUploadPage => error_upload_page(locale),
            Templates::ListPage(user, total_notes) => list_page(user, total_notes, locale),
            Templates::EraseAllNotesPage(user_id) => erase_all_notes_page(user_id, locale),
            Templates::SuccessEraseAllNotesPage(user_id) => {
                success_erase_all_notes_page(user_id, locale)
            }
            Templates::ErrorEraseAllNotesPage => error_erase_all_notes_page(locale),
            Templates::LanguagePage => language_page(&repo_url, locale),
            Templates::HelpPage => help_page(locale),
            Templates::CreditsPage => {
                credits_page(&author, &profile_name, &profile_url, &repo_url, locale)
            }
            Templates::UnsupportedInputPage(input) => unsupported_input_page(input, locale),
        }
    }
}

fn start_page(user: &str, locale: &str) -> String {
    debug!("rendering with locale: {:?}", locale);
    format!("{}", t!("start_page", locale = locale, user = user))
}

fn random_note_page(user: &str, locale: &str) -> String {
    format!("{}", t!("random_note_page", locale = locale, user = user))
}

fn delete_note_page(note_id: &str, locale: &str) -> String {
    format!(
        "{}",
        t!("delete_note_page", locale = locale, note_id = note_id)
    )
}

fn success_delete_note_page(note_id: &str, locale: &str) -> String {
    format!(
        "{}",
        t!(
            "success_delete_note_page",
            locale = locale,
            note_id = note_id
        )
    )
}

fn error_delete_note_page(locale: &str) -> String {
    format!("{}", t!("error_delete_note_page", locale = locale))
}

fn loading_page(locale: &str) -> String {
    format!("{}", t!("loading_page", locale = locale))
}

fn upload_page(
    user: &str,
    total_notes: &str,
    total_users: &str,
    user_videonotes_list: &Vec<UserStats>,
    locale: &str,
) -> String {
    let mut stats_list = String::new();

    for stat in user_videonotes_list {
        let new_stat_entry = format!(
            "{}{}",
            stats_list,
            t!(
                "extra_page_stat_entry",
                locale = locale,
                user = stat.username,
                user_total_notes = stat.videonotes,
            )
        );

        stats_list = new_stat_entry;
    }

    format!(
        "{}",
        t!(
            "upload_page",
            locale = locale,
            user = user,
            total_notes = total_notes,
            total_users = total_users,
            user_videonotes_list = stats_list,
        )
    )
}

fn success_upload_page(locale: &str) -> String {
    format!("{}", t!("success_upload_page", locale = locale))
}

fn error_upload_page(locale: &str) -> String {
    format!("{}", t!("error_upload_page", locale = locale))
}

fn list_page(user: &str, total_notes: &str, locale: &str) -> String {
    format!(
        "{}",
        t!(
            "list_page",
            locale = locale,
            user = user,
            total_notes = total_notes
        )
    )
}

fn erase_all_notes_page(user_id: &str, locale: &str) -> String {
    format!(
        "{}",
        t!("erase_all_notes_page", locale = locale, user_id = user_id)
    )
}

fn success_erase_all_notes_page(note_id: &str, locale: &str) -> String {
    format!(
        "{}",
        t!(
            "success_erase_all_notes_page",
            locale = locale,
            note_id = note_id
        )
    )
}

fn error_erase_all_notes_page(locale: &str) -> String {
    format!("{}", t!("error_erase_all_notes_page", locale = locale))
}

fn credits_page(
    author: &str,
    profile_name: &str,
    profile_url: &str,
    repo_url: &str,
    locale: &str,
) -> String {
    format!(
        "{}",
        t!(
            "credits_page",
            locale = locale,
            author = author,
            profile_name = profile_name,
            profile_url = profile_url,
            repo_url = repo_url,
        )
    )
}

fn language_page(repo_url: &str, locale: &str) -> String {
    format!(
        "{}",
        t!("language_page", locale = locale, repo_url = repo_url)
    )
}

fn help_page(locale: &str) -> String {
    format!("{}", t!("help_page", locale = locale))
}

fn unsupported_input_page(input_type: &str, locale: &str) -> String {
    let media_input = match input_type {
        "photo" => Some(format!(
            "{}",
            t!("unsupported_page.media.photo", locale = locale)
        )),
        "video" => Some(format!(
            "{}",
            t!("unsupported_page.media.video", locale = locale)
        )),
        "voice" => Some(format!(
            "{}",
            t!("unsupported_page.media.voice", locale = locale)
        )),
        "audio" => Some(format!(
            "{}",
            t!("unsupported_page.media.audio", locale = locale)
        )),
        "document" => Some(format!(
            "{}",
            t!("unsupported_page.media.document", locale = locale)
        )),
        _ => None,
    };

    if let Some(media) = media_input {
        return format!(
            "{}",
            t!("unsupported_page", locale = locale, media_type = &media)
        );
    }

    format!("{}", t!("unsupported_page.other_media", locale = locale))
}
