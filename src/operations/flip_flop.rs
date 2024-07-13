use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn flip(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let result = ffmpeg_operations::flip_video(v?)?;
            Ok(MediaObject::Encoded(result))
        } else {
            let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
            dyn_images.iter_images_mut(|f, _| {
                let mut fb = FrameBufferOwned::new_from_dyn_image(f);
                ops::flip::vertical(fb.fb_mut());
                fb.into_dyn_image()
            });

            Ok(MediaObject::DynamicImages(dyn_images))
        }
    }

    pub fn flop(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let result = ffmpeg_operations::flop_video(v?)?;
            Ok(MediaObject::Encoded(result))
        } else {
            let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
            dyn_images.iter_images_mut(|f, _| {
                let mut fb = FrameBufferOwned::new_from_dyn_image(f);
                ops::flip::horizontal(fb.fb_mut());
                fb.into_dyn_image()
            });

            Ok(MediaObject::DynamicImages(dyn_images))
        }
    }
}
