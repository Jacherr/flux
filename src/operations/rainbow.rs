use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn rainbow(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::rainbow_video(v?).map(MediaObject::Encoded);
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        if dyn_images.images.len() == 1 {
            dyn_images.images = dyn_images.images.into_iter().cycle().take(75).collect::<Vec<_>>();
        };

        let count = dyn_images.images.len();

        dyn_images.iter_images_mut(|f, i| {
            let mut fb = FrameBufferOwned::new_from_dyn_image(f);
            ops::filter::hue_rotate(fb.fb_mut(), (i * 360 / count) as f32);
            fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
