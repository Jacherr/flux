use image::codecs::gif::Repeat;
use image::GenericImageView;
use serde::Serialize;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::{get_video_dimensions, get_video_fps, get_video_frame_count, get_video_length};
use crate::processing::filetype::{get_sig, get_sig_incl_mp4, Type};
use crate::processing::gif::gif_get_comments;
use crate::processing::media_object::MediaObject;

#[derive(Serialize)]
pub struct ImageInfo {
    pub file_size_bytes: u64,
    pub mime_type: String,
    pub dimensions: String,
    pub frame_count: Option<u64>,
    pub repeat: Option<String>,
    pub comments: Vec<String>,
}

#[derive(Serialize)]
pub struct VideoInfo {
    pub file_size_bytes: u64,
    pub mime_type: String,
    pub dimensions: String,
    pub duration_ms: u64,
    pub frame_count: u64,
    pub fps: f64,
}

#[derive(Serialize)]
pub enum MediaInfo {
    Image(ImageInfo),
    Video(VideoInfo),
}

impl MediaContainer {
    /// Get some metadata about an image or video.
    pub fn info(&self) -> Result<MediaInfo, FluxError> {
        let input = self.pop_input()?;
        if let Some(v) = input.try_encoded_video(true).map(|v| v.unwrap()) {
            let file_size_bytes = v.len() as u64;
            let mime_type = get_sig_incl_mp4(v).unwrap().as_mime().to_owned();
            let dimensions = get_video_dimensions(v).map(|(w, h)| format!("{w}x{h}"))?;
            let duration_ms = get_video_length(v)?.as_millis() as u64;
            let frame_count = get_video_frame_count(v)? as u64;
            let fps = get_video_fps(v)?;

            Ok(MediaInfo::Video(VideoInfo {
                file_size_bytes,
                mime_type,
                dimensions,
                duration_ms,
                frame_count,
                fps,
            }))
        } else if let MediaObject::Encoded(ref e) = input {
            let file_size_bytes = e.len() as u64;
            let mime_type = get_sig(e).map(|s| s.as_mime()).unwrap_or("unknown").to_owned();
            let dyn_images = input.to_dynamic_images(&self.limits)?;
            let dimensions = dyn_images
                .images
                .first()
                .map(|d| {
                    let d = d.0.dimensions();
                    format!("{}x{}", d.0, d.1)
                })
                .unwrap_or("0x0".to_owned());
            let frame_count = if dyn_images.images.len() > 1 {
                Some(dyn_images.images.len() as u64)
            } else {
                None
            };

            let repeat = if let Repeat::Finite(n) = dyn_images.repeat
                && dyn_images.images.len() > 1
            {
                Some(format!("{n} times"))
            } else if dyn_images.images.len() > 1 {
                Some("Infinite".to_string())
            } else {
                None
            };

            let comments = if get_sig(e) == Some(Type::Gif) {
                gif_get_comments(e)
            } else {
                vec![]
            };

            Ok(MediaInfo::Image(ImageInfo {
                file_size_bytes,
                mime_type,
                dimensions,
                frame_count,
                repeat,
                comments,
            }))
        } else {
            Err(FluxError::Other(
                "Getting metadata is only supported as a first step.".to_owned(),
            ))
        }
    }
}
