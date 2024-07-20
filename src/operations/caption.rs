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
    pub fn caption(&self, text: &str, bottom: bool, black: bool) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(input) = input.try_encoded_video(self.limits.video_decode_permitted) {
            let input = input?;
            let (w, _) = get_video_dimensions(input)?;
            let mut text = vips_generate_caption(text, w)?;
            if black {
                text.invert();
            }

            let res = ffmpeg_operations::caption_video(input, text, bottom)?;

            Ok(MediaObject::Encoded(res))
        } else {
            let mut x = input.to_dynamic_images(&self.limits)?.into_owned();
            let (w, h) = x.maybe_first()?.0.dimensions();
            let mut text = vips_generate_caption(text, w as usize)?;
            if black {
                text.invert();
            }

            let text_owned_fb = FrameBufferOwned::new_from_dyn_image(&text);
            let text_fb = text_owned_fb.fb();

            x.iter_images_mut(|f, _| {
                let img_fb = FrameBufferOwned::new_from_dyn_image(f);

                let mut canvas = framebuffer::new(w as usize, (h + text.height()) as usize);

                if bottom {
                    ops::overlay::replace(&mut canvas, img_fb.fb(), 0, 0);
                    ops::overlay::replace(&mut canvas, text_fb, 0, img_fb.height as isize);
                } else {
                    ops::overlay::replace(&mut canvas, text_fb, 0, 0);
                    ops::overlay::replace(&mut canvas, img_fb.fb(), 0, text.height() as isize);
                };

                let w = canvas.width as u32;
                let h = canvas.height as u32;
                framebuffer_to_dyn_image(w, h, canvas.into_vec())
            });

            Ok(MediaObject::DynamicImages(x))
        }
    }
}
