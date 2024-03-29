use dotenvy as dotenv;

#[derive(Debug, Clone)]
pub enum Templates {
    StartPage(String),
    ExtraPage(String, String, String, String),
    HelpPage,
    CreditsPage,
    UnsupportedInputPage(String),
}

impl Templates {
    pub fn render(&self) -> String {
        let author =
            dotenv::var("AUTHOR").expect("error loading author from envirenment variables");

        let profile_name = dotenv::var("AUTHOR_PROFILE_NAME")
            .expect("error loading author from envirenment variables");

        let profile_url = dotenv::var("AUTHOR_PROFILE_URL")
            .expect("error loading author profile url from envirenment variables");

        let repo_url = dotenv::var("CODE_REPO_URL")
            .expect("error loading code repo url from envirenment variables");

        match self {
            Templates::StartPage(user) => start_page(user),
            Templates::ExtraPage(user, total_notes, total_users, user_videonotes_list) => {
                extra_page(user, total_notes, total_users, user_videonotes_list)
            }
            Templates::HelpPage => help_page(),
            Templates::CreditsPage => credits_page(&author, &profile_name, &profile_url, &repo_url),
            Templates::UnsupportedInputPage(input) => unsupported_input_page(input),
        }
    }
}

fn start_page(user: &str) -> String {
    format!("{}", t!("start_page", user = user))
}

fn extra_page(
    user: &str,
    total_notes: &str,
    total_users: &str,
    user_videonotes_list: &str,
) -> String {
    format!(
        "{}",
        t!(
            "extra_page",
            user = user,
            total_notes = total_notes,
            total_users = total_users,
            user_videonotes_list = user_videonotes_list
        )
    )
}

fn credits_page(author: &str, profile_name: &str, profile_url: &str, repo_url: &str) -> String {
    format!(
        "{}",
        t!(
            "credits_page",
            author = author,
            profile_name = profile_name,
            profile_url = profile_url,
            repo_url = repo_url
        )
    )
}

fn help_page() -> String {
    format!("{}", t!("help_page"))
}

fn unsupported_input_page(input_type: &str) -> String {
    let media_input = match input_type {
        "photo" => Some("a photo"),
        "video" => Some("a video"),
        "voice" => Some("a voice recording"),
        "audio" => Some("an audio file"),
        "document" => Some("a document"),
        _ => None,
    };

    if let Some(input) = media_input {
        return format!(
            r"⚠️ <b>WARNING</b> ⚠️
<b>This bot can't receive a {}. Check /help for instructions.</b>",
            input
        );
    }

    format!(
        r"⚠️ <b>WARNING</b> ⚠️
<b>Unsupported media. Check /help for instructions.</b>"
    )
}
