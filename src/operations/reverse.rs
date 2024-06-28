use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::core::media_object::{DynamicImagesMediaObject, MediaObject};
use crate::processing::ffmpeg::ffmpeg_operations;

impl MediaContainer {
    pub fn reverse(&self) -> Result<MediaObject, FluxError> {
        let input = self.pop_input()?;
        let out = if !input.is_encoded_video() {
            let input = input.to_dynamic_images()?;
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
