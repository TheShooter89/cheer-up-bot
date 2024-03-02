use log;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    log::info!("Starting throw dice bot...");
    dotenvy::dotenv().ok();

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        log::info!("Throwing dice bot...");
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
