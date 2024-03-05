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
