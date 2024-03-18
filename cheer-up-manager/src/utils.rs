use dotenvy as dotenv;

use teloxide::types::Chat;
use tokio::fs;

pub async fn create_user_folder(user: &Chat) -> Result<String, std::io::Error> {
    let folder_path = get_user_folder_path(&user);
    println!("creating folder '{:?}'", folder_path);
    fs::create_dir_all(&folder_path).await?;
    Ok(folder_path)
}

pub fn get_user_folder_path(user: &Chat) -> String {
    let folder_path = format!(
        "{}/videonotes/{}_{}",
        dotenv::var("COMMON_DATA_FOLDER").unwrap_or("_EXTRA/".to_string()),
        user.id,
        user.username().unwrap_or("none")
    );

    folder_path
}
