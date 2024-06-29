use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::{DynamicImagesMediaObject, MediaObject};

use super::OperationResult;

impl MediaContainer {
    pub fn reverse(&self) -> OperationResult {
        let input = self.pop_input()?;
        let out = if !input.is_encoded_video() {
            let input = input.to_dynamic_images(self.frame_limit)?;
            if input.images.len() == 1 {
                return Err(FluxError::InputImageError(
                    "Reversing images requires more than one frame".to_string(),
                ));
            }

            let audio = input.audio.clone();
            let repeat = input.repeat;

            let new = input.images.clone().into_iter().rev().collect::<Vec<_>>();
            MediaObject::DynamicImages(DynamicImagesMediaObject {
                images: new,
                audio,
                repeat,
            })
        } else {
            let out = ffmpeg_operations::reverse_video(input.unwrap_encoded())?;
            MediaObject::Encoded(out)
        };

        Ok(out)
    }
}
