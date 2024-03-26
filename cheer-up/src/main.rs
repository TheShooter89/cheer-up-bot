// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales", fallback = "en");
use rust_i18n::set_locale;

// use log;
use teloxide::{prelude::*, RequestError};

mod commands;
mod messages;
mod templates;
mod user;
mod utils;
mod videonotes;

async fn handle_callback(bot: Bot, query: CallbackQuery) -> Result<(), RequestError> {
    let cq_data = match query.data {
        Some(data) => data,
        None => "none".to_string(),
    };
    println!("callback query data is: {:#?}", cq_data);
    bot.answer_callback_query(query.id).await?;

    bot.send_message(query.message.unwrap().chat.id, cq_data)
        .await?;

    Ok(())
}

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
        .branch(Update::filter_callback_query().endpoint(handle_callback));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
