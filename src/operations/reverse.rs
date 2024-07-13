use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::{DynamicImagesMediaObject, MediaObject};

use super::OperationResult;

impl MediaContainer {
    pub fn reverse(&self) -> OperationResult {
        let input = self.pop_input()?;
        let out = if let Some(input) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let out = ffmpeg_operations::reverse_video(input?)?;
            MediaObject::Encoded(out)
        } else {
            let input = input.to_dynamic_images(&self.limits)?;
            if input.images.len() == 1 {
                return Err(FluxError::SinglePageMediaUnsupported);
            }

            let audio = input.audio.clone();
            let repeat = input.repeat;

            let new = input.images.clone().into_iter().rev().collect::<Vec<_>>();
            MediaObject::DynamicImages(DynamicImagesMediaObject {
                images: new,
                audio,
                repeat,
            })
        };

        Ok(out)
    }
}
