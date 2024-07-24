use std::time::Duration;

use image::{Delay, GenericImageView};

use crate::core::media_container::MediaContainer;
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
        let (w, h) = dyn_images.maybe_first()?.0.dimensions();
        let repeat = dyn_images.repeat;

        let frames = dyn_images
            .into_owned()
            .into_images()
            .into_iter()
            .map(|x| (x.0, Delay::from_saturating_duration(x.1.unwrap_or(Duration::default()))))
            .collect::<Vec<_>>();

        let out = crate::processing::encode::gif::encode(frames, w as u16, h as u16, repeat)?;

        Ok(MediaObject::Encoded(out))
    }
}
