use image::GenericImageView;

use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::{framebuffer, ops};
use crate::processing::ffmpeg::{ffmpeg_operations, get_video_dimensions};
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;
use crate::vips::vips_generate_caption;

use super::OperationResult;

impl MediaContainer {
    pub fn caption(&self, text: &str) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(input) = input.try_encoded_video() {
            let (w, _) = get_video_dimensions(input)?;
            let text = vips_generate_caption(text, w)?;
            let res = ffmpeg_operations::caption_video(input, text)?;

            Ok(MediaObject::Encoded(res))
        } else {
            let mut x = input.to_dynamic_images(&self.limits)?.into_owned();
            let (w, h) = x.images.first().unwrap().0.dimensions();
            let text = vips_generate_caption(text, w as usize)?;

            let text_owned_fb = FrameBufferOwned::new_from_dyn_image(&text);
            let text_fb = text_owned_fb.fb();

            x.iter_images_mut(|f, _| {
                let img_fb = FrameBufferOwned::new_from_dyn_image(f);

                let mut canvas = framebuffer::new(w as usize, (h + text.height()) as usize);
                ops::overlay::replace(&mut canvas, text_fb, 0, 0);
                ops::overlay::replace(&mut canvas, img_fb.fb(), 0, text.height() as isize);

                let w = canvas.width as u32;
                let h = canvas.height as u32;
                framebuffer_to_dyn_image(w, h, canvas.into_vec())
            });

            Ok(MediaObject::DynamicImages(x))
        }
    }
}
