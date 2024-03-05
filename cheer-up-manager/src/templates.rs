#[derive(Debug)]
pub enum Templates {
    StartPage,
    HelpPage,
    CreditsPage,
    EraseConfirmationPage,
}

impl Templates {
    pub fn render(&self) -> String {
        match self {
            Templates::StartPage => "Start page".to_string(),
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
    format!(
        r"<b>Help & Commands</b>
This bot handles video notes only. If you enter any textual message bot will respond with starting welcome message, sending other media (picture, audio, regular videos, and so on) will fail with an error message

Available commands:
<code>/start</code> - get the bot starting page
<code>/list</code> - list all video notes you uploaded
<code>/erase</code> - ⚠️ delete all video notes at once ⚠️
<code>/credits</code> - show bot credits with author profile and code repository links"
    )
}
