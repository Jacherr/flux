use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn blur(&self, strength: Option<f32>) -> OperationResult {
        let input = self.pop_input()?;
        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            let mut fb = FrameBufferOwned::new_from_dyn_image(f);
            ops::blur::gaussian(fb.fb_mut(), strength.unwrap_or(3.0));
            fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
