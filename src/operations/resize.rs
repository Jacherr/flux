use image::GenericImageView;

use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::{ffmpeg_operations, get_video_dimensions};
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;

use super::OperationResult;

pub struct ResizeOptions {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub scale: Option<f32>,
}

impl MediaContainer {
    pub fn resize(&self, options: ResizeOptions) -> OperationResult {
        let input = self.pop_input()?;

        let (mut width, mut height) = if let Some(input) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let (w, h) = get_video_dimensions(input?)?;
            (w as u64, h as u64)
        } else {
            let d = input
                .to_dynamic_images(&self.limits)?
                .images
                .first()
                .unwrap()
                .0
                .dimensions();
            (d.0 as u64, d.1 as u64)
        };

        match options.scale {
            Some(s) => {
                width = (width as f32 * s).floor() as u64;
                height = (height as f32 * s).floor() as u64;
            },
            None => {
                width = options.width.unwrap_or(width * 2);
                height = options.height.unwrap_or(height * 2);
            },
        }

        let out = if let Some(input) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let real_width = if width % 2 == 1 { width + 1 } else { width };
            let real_height = if height % 2 == 1 { height + 1 } else { height };
            let out = ffmpeg_operations::resize_video(input?, real_width as usize, real_height as usize)?;
            MediaObject::Encoded(out)
        } else {
            let mut input = input.to_dynamic_images(&self.limits)?.into_owned();

            input.iter_images_mut(|f, _| {
                let fb = FrameBufferOwned::new_from_dyn_image(f);
                let out = ops::resize::nearest(fb.fb(), width.clamp(2, 2048) as usize, height.clamp(2, 2048) as usize);

                let w = out.width as u32;
                let h = out.height as u32;
                framebuffer_to_dyn_image(w, h, out.into_vec())
            });

            MediaObject::DynamicImages(input)
        };

        Ok(out)
    }
}
