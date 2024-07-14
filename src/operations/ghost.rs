use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn ghost(&self, depth: Option<u64>) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let c = dyn_images.clone();

        if dyn_images.images.len() == 1 {
            return Err(FluxError::SinglePageMediaUnsupported);
        };

        let depth = depth.map(|d| d.clamp(1, 20)).unwrap_or(3);
        let change = (u8::MAX / depth as u8 / 8).clamp(1, u8::MAX);

        dyn_images.iter_images_mut(|f, i| {
            let mut next_div = 2;

            let mut fb = FrameBufferOwned::new_from_dyn_image(f);

            for idx in 0..depth {
                let ghost = c.get_circular_neg(i as isize - (idx as isize + 1));
                let mut ghost_fb = FrameBufferOwned::new_from_dyn_image(&ghost.0);
                ops::filter::opacity(ghost_fb.fb_mut(), 1.0 / next_div as f32);

                ops::overlay::blend(fb.fb_mut(), ghost_fb.fb(), 0, 0);
                next_div += change;
            }

            fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
