use dotenvy as dotenv;

use teloxide::{
    net::Download,
    requests::{Requester, ResponseResult},
    types::{Chat, VideoNote},
    Bot,
};
use tokio::fs;

use crate::utils::create_user_folder;

pub async fn upload_vnote(bot: &Bot, videonote: &VideoNote, chat: &Chat) -> ResponseResult<()> {
    let folder_path = create_user_folder(&chat).await?;
    println!("created user folder path is: {}", folder_path);

    let vnote_file = bot.get_file(&videonote.file.id).await?;
    let vnote_out_path = format!("{}/{}.mpeg", folder_path, videonote.file.id);

    let mut output_file = fs::File::create(&vnote_out_path).await?;

    bot.download_file(&vnote_file.path, &mut output_file)
        .await?;

    Ok(())
}
