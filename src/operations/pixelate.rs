use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::{ffmpeg_operations, get_video_dimensions};
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;

use super::OperationResult;

impl MediaContainer {
    pub fn pixelate(&self, strength: Option<f32>) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let v = v?;

            let (v_w, v_h) = get_video_dimensions(v)?;
            let strength = strength.unwrap_or(10.0) + 50.0;

            let (pix_w, pix_h) = (v_w as f32 / strength, v_h as f32 / strength);

            ffmpeg_operations::pixelize_video(v, pix_w as u64, pix_h as u64).map(MediaObject::Encoded)
        } else {
            let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

            let px_height = dyn_images.maybe_first()?.0.height() as f32 / strength.unwrap_or(10.0) as f32;
            dyn_images.iter_images_mut(|f, _| {
                let new_width = (f.width() as f32 * (px_height / f.height() as f32)).round() as u32;

                let fb = FrameBufferOwned::new_from_dyn_image(f);
                let smaller = ops::resize::nearest(fb.fb(), new_width as usize, px_height as usize);
                let new = ops::resize::nearest(&smaller, f.width() as usize, f.height() as usize);

                framebuffer_to_dyn_image(f.width(), f.height(), new.into_vec())
            });
            Ok(MediaObject::DynamicImages(dyn_images))
        }
    }
}
