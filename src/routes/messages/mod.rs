use std::error::Error;
use std::fs::File;
use std::io::Write;

use teloxide::prelude::{Message, Requester};
use teloxide::types::{ChatId, MediaKind, MediaPhoto, MediaVideo, MessageKind};
use teloxide::Bot;

pub async fn handle_image_video_message(
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let MessageKind::Common(ref common_msg) = message.kind {
        match &common_msg.media_kind {
            MediaKind::Photo(photo) => process_image(&bot, &message.chat.id, photo).await?,
            MediaKind::Video(video) => process_video(&bot, &message.chat.id, video).await?,
            MediaKind::Text(_text) => {
                bot.send_message(
                    message.chat.id,
                    "Your question will be sent to the admin. Are you want to do this?",
                )
                .await?;
            }
            _ => {
                bot.send_message(message.chat.id, "Unsupported media type")
                    .await?;
            }
        }
    }
    Ok(())
}

async fn process_image(
    bot: &Bot,
    chat_id: &ChatId,
    photo_media: &MediaPhoto,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let photo = match photo_media.photo.iter().max_by_key(|p| p.width * p.height) {
        Some(photo) => photo,
        None => {
            log::info!("No photos found");
            return Ok(());
        }
    };

    let file = bot.get_file(&photo.file.id).await?;
    let data = download_file(&bot, &file.path).await?;
    let file_name = format!("photo_{}.jpg", file.id);
    save_file_locally(&file_name, &data).await?;

    bot.send_message(*chat_id, "Image saved").await?;
    Ok(())
}

async fn process_video(
    bot: &Bot,
    chat_id: &ChatId,
    video_media: &MediaVideo,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let file = bot.get_file(&video_media.video.file.id).await?;
    let data = download_file(&bot, &file.path).await?;
    let file_name = &video_media
        .video
        .file_name
        .clone()
        .unwrap_or(format!("video_{}.mp4", file.id));
    save_file_locally(file_name, &data).await?;

    bot.send_message(*chat_id, "Video saved").await?;
    Ok(())
}

async fn download_file(
    bot: &Bot,
    file_path: &str,
) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let file_url = format!(
        "https://api.telegram.org/file/bot{}/{}",
        bot.token(),
        file_path
    );
    let resp = reqwest::get(&file_url).await?;
    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}

async fn save_file_locally(
    filename: &str,
    data: &[u8],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = format!("./wed_files/{}", filename);
    let mut file = File::create(filename)?;
    file.write(data)?;
    Ok(())
}
