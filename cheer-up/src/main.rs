// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales", fallback = "en");
use rust_i18n::set_locale;

// use log;
use teloxide::{prelude::*, RequestError};

mod buttons;
mod callbacks;
mod commands;
mod keyboards;
mod messages;
mod stats;
mod templates;
mod user;
mod utils;
mod videonotes;

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let app_locale = dotenvy::var("LOCALE").unwrap_or("en".to_string());
    set_locale(&app_locale);

    let bot = Bot::from_env();

    // teloxide::repl(bot, handle_input).await;
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(messages::handle_message))
        .branch(Update::filter_callback_query().endpoint(callbacks::handle_callback));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
