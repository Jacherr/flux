use image::GenericImageView;

use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::ffmpeg::{ffmpeg_operations, get_video_dimensions};
use crate::processing::framebuffer::FrameBuffer;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;

use super::OperationResult;

pub struct ResizeOptions {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub scale: Option<f32>,
}

impl MediaContainer {
    pub fn resize(&self, options: ResizeOptions) -> OperationResult {
        let input = self.pop_input()?;

        let (mut width, mut height) = if let Some(input) = input.try_encoded_video() {
            get_video_dimensions(input)?
        } else {
            let d = input
                .to_dynamic_images(&self.limits)?
                .images
                .first()
                .unwrap()
                .0
                .dimensions();
            (d.0 as usize, d.1 as usize)
        };

        match options.scale {
            Some(s) => {
                width = (width as f32 * s).floor() as usize;
                height = (height as f32 * s).floor() as usize;
            },
            None => {
                width = options.width.unwrap_or(width * 2);
                height = options.height.unwrap_or(height * 2);
            },
        }

        let out = if let Some(input) = input.try_encoded_video() {
            let out = ffmpeg_operations::resize_video(input, width, height)?;
            MediaObject::Encoded(out)
        } else {
            let mut input = input.to_dynamic_images(&self.limits)?.into_owned();

            input.iter_images_mut(|f, _| {
                let fb = FrameBuffer::new_from_dyn_image(f);
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
