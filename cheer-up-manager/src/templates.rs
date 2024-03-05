#[derive(Debug)]
pub enum Templates {
    StartPage(String),
    HelpPage,
    CreditsPage,
    EraseConfirmationPage,
}

impl Templates {
    pub fn render(&self) -> String {
        match self {
            Templates::StartPage(user) => start_page(user).to_string(),
            Templates::HelpPage => "Help page".to_string(),
            Templates::CreditsPage => "Credits page".to_string(),
            Templates::EraseConfirmationPage => "EraseConfirmation page".to_string(),
        }
    }
}

fn start_page(user: &str) -> String {
    format!(
        r"<b>Hi {user}!</b>
This bot is a manitenance bot used to upload and handle your video notes for CheerUp bot so that your friend can use the main CheerUp bot to receive a random video notes from all video notes you uploaded using this maintenance bot
Just send a video note (aka bubble videos) to this bot and it will be ready to go

To get every video note you already type <code>/list</code>
For further help and additional commands type <code>/help</code>

created with 💛️💙️ by tanque - see <code>/credits</code>"
    )
}

fn help_page() -> String {
    r"<b>Help & Commands</b>
This bot handles video notes only. If you enter any textual message bot will respond with starting welcome message, sending other media (picture, audio, regular videos, and so on) will fail with an error message

Available commands:
<code>/start</code> - get the bot starting page
<code>/list</code> - list all video notes you uploaded
<code>/erase</code> - ⚠️ delete all video notes at once ⚠️
<code>/credits</code> - show bot credits with author profile and code repository links".to_string()
}

fn credits_page(author: &str, profile_name: &str, profile_url: &str, repo_url: &str) -> String {
    format!(
        r"<b>Help & Commands</b>
This bot has been created in March 2024 by {author} as open source software, all code is published on Github

Author: {profile_name} - {profile_url}
Code: {repo_url}

#supportukraine
Author stands with ukrainian people in their fight for Freedom and Peace - visit https://stand-with-ukraine.pp.ua/ for a list of organizations you can support and donate to"
    )
}

fn erase_confirmation_page() -> String {
    r"🚨️ <b>ERASE ALL VIDEO NOTES</b> 🚨️

⚠️ <b>WARNING</b> ⚠️
This operation will <b>permanently delete all your notes</b>

Do you confirm?"
        .to_string()
}
