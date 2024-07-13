use image::codecs::gif::Repeat;
use image::load_from_memory;

use crate::core::media_container::MediaContainer;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::ffmpeg::{april_fools, get_video_first_frame};
use crate::processing::media_object::{DynamicImagesMediaObject, MediaObject};

use super::OperationResult;

impl MediaContainer {
    pub fn april_fools(&self) -> OperationResult {
        let input = self.pop_input()?;
        let first = if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            DynamicImageWrapper::new(load_from_memory(&get_video_first_frame(v?)?)?, None)
        } else {
            input.to_dynamic_images(&self.limits)?.into_owned().images[0].clone()
        };

        let image = MediaObject::DynamicImages(DynamicImagesMediaObject {
            images: vec![first],
            audio: None,
            repeat: Repeat::Infinite,
        })
        .encode(&self.limits)?;

        let out = april_fools(image)?;

        Ok(MediaObject::Encoded(out))
    }
}
