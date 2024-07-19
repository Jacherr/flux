use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn invert(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::invert_video(v?).map(MediaObject::Encoded);
        }

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        dyn_images.iter_images_mut(|f, _| {
            let mut o = f.clone();
            o.invert();
            o
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
