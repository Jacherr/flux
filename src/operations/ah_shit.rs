use image::imageops::FilterType;

use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ah_shit;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn ah_shit(&self) -> OperationResult {
        let input = self.pop_input()?;
        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        // resize to fit
        dyn_images.iter_images_mut(|f, _| f.resize(1280, 720, FilterType::Gaussian));
        let enc = MediaObject::DynamicImages(dyn_images).encode(&self.limits)?;

        let out = ah_shit(enc)?;

        Ok(MediaObject::Encoded(out))
    }
}
