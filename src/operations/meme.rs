use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::vips::vips_generate_meme_text;

use super::OperationResult;

impl MediaContainer {
    pub fn meme(&self, text_top: Option<String>, text_bottom: Option<String>) -> OperationResult {
        let input = self.pop_input()?;

        let mut input = input.to_dynamic_images(&self.limits)?.into_owned();
        let first = &input.maybe_first()?.0;

        let width = first.width();
        let target_height = first.height() as usize / 5;

        let top_text_image = if let Some(t) = text_top
            && !t.is_empty()
        {
            Some(vips_generate_meme_text(&t, width as usize, target_height)?)
        } else {
            None
        };

        let bottom_text_image = if let Some(b) = text_bottom
            && !b.is_empty()
        {
            Some(vips_generate_meme_text(&b, width as usize, target_height)?)
        } else {
            None
        };

        if top_text_image.is_none() && bottom_text_image.is_none() {
            return Err(FluxError::ParameterError(
                "Either `top' or `bottom' option must be defined to add meme text to the input".to_owned(),
            ));
        }

        input.iter_images_mut(|f, _| {
            let h = f.height();
            let mut fb = FrameBufferOwned::new_from_dyn_image(f);
            if top_text_image.is_some() {
                let top_text_image = top_text_image.clone().unwrap();
                let top_text_image_fb = FrameBufferOwned::new_from_dyn_image(&top_text_image);
                ops::overlay::blend(fb.fb_mut(), &top_text_image_fb, 0, 0);
            }
            if bottom_text_image.is_some() {
                let bottom_text_image = bottom_text_image.clone().unwrap();
                let b_h = bottom_text_image.height();
                let bottom_text_image_fb = FrameBufferOwned::new_from_dyn_image(&bottom_text_image);
                ops::overlay::blend(fb.fb_mut(), &bottom_text_image_fb, 0, (h - b_h) as isize);
            }

            fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(input))
    }
}
