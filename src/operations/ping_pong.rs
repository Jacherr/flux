use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn ping_pong(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return Ok(MediaObject::Encoded(ffmpeg_operations::gloop_video(v?)?));
        } else if let MediaObject::DynamicImages(ref d) = input
            && d.images.len() == 1
        {
            return Err(FluxError::SinglePageMediaUnsupported);
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let rev_iter = dyn_images.images.clone().into_iter().rev();
        dyn_images.images.extend(rev_iter);

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
