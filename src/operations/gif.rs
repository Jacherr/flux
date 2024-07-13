use image::{Delay, GenericImageView};

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::encode::gif::encode;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::filetype::{get_sig, Type};
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn gif(&self) -> OperationResult {
        let input = self.pop_input()?;
        if let MediaObject::Encoded(ref e) = input
            && get_sig(e) == Some(Type::Gif)
        {
            return Ok(input);
        } else if let Some(e) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::video_to_gif(e?).map(MediaObject::Encoded);
        }

        let dyn_images = input.to_dynamic_images(&self.limits)?;
        let (w, h) = dyn_images
            .images
            .get(0)
            .ok_or(FluxError::CorruptInput("Input has no frames".to_owned()))?
            .0
            .dimensions();

        let frames = dyn_images
            .images
            .iter()
            .map(|d| (&d.0, Delay::from_saturating_duration(d.1.unwrap_or_default())))
            .collect::<Vec<_>>();

        let enc = encode(frames, w as u16, h as u16, dyn_images.repeat)?;

        Ok(MediaObject::Encoded(enc))
    }
}
