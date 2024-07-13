use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations::scramble_video;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn scramble(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let s = scramble_video(v?)?;

            return Ok(MediaObject::Encoded(s));
        }

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        if dyn_images.images.len() == 1 {
            return Err(FluxError::SinglePageMediaUnsupported);
        };

        dyn_images.images.shuffle(&mut thread_rng());
        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
