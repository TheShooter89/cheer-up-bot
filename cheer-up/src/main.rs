use log;
use teloxide::{net::Download, prelude::*, types::InputFile};
use tokio::fs;

#[tokio::main]
async fn main() {
    log::info!("Starting throw dice bot...");
    dotenvy::dotenv().ok();

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        log::info!("Throwing dice bot...");
        println!("Throwing dice bot...");
        println!("msg.video_note(): {:?}", msg.video_note());
        match msg.video_note() {
            Some(vnote) => {
                //
                let vnote_file_id = vnote.file.id.clone();
                println!("vnote: {:?}", vnote);
                println!("vnote.id: {:?}", vnote.file.id);

                bot.send_video_note(msg.chat.id, InputFile::file_id(vnote_file_id))
                    .await?;

                println!("vnote sent to {:?}", msg.chat.id.to_string())
            }
            None => println!("msg is not a video note"),
        }
        // bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
