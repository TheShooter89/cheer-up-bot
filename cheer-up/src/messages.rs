use teloxide::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    VideoNote,
    Text,
    Photo,
    Video,
    Voice,
    Audio,
    Document,
    Unknown,
}

impl MessageType {
    pub fn from_msg(msg: &Message) -> MessageType {
        if let Some(_) = msg.video_note() {
            return MessageType::VideoNote;
        }

        if let Some(_) = msg.text() {
            return MessageType::Text;
        }

        if let Some(_) = msg.photo() {
            return MessageType::Photo;
        }

        if let Some(_) = msg.video() {
            return MessageType::Video;
        }

        if let Some(_) = msg.voice() {
            return MessageType::Voice;
        }

        if let Some(_) = msg.audio() {
            return MessageType::Audio;
        }

        if let Some(_) = msg.document() {
            return MessageType::Document;
        }

        MessageType::Unknown
    }

    pub fn name(&self) -> &str {
        match self {
            MessageType::VideoNote => "videonote",
            MessageType::Text => "text",
            MessageType::Photo => "photo",
            MessageType::Video => "video",
            MessageType::Voice => "voice",
            MessageType::Audio => "audio",
            MessageType::Document => "document",
            MessageType::Unknown => "unknown",
        }
    }
}
