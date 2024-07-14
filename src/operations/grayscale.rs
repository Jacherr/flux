use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn grayscale(&self) -> OperationResult {
        let input = self.pop_input()?;
        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::grayscale_video(v?).map(MediaObject::Encoded);
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        dyn_images.iter_images_mut(|f, _| {
            let mut fb = FrameBufferOwned::new_from_dyn_image(f);
            ops::filter::grayscale(fb.fb_mut(), 1.0);
            fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
