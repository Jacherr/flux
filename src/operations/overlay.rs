use image::imageops::FilterType;
use image::GenericImageView;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn overlay(&self) -> OperationResult {
        let base = self.pop_input()?;
        let overlay = self.pop_input()?;

        if base.is_encoded_video() || overlay.is_encoded_video() {
            return Err(FluxError::InputMediaError(
                "Overlay does not support videos.".to_owned(),
            ));
        }

        let mut base_dyn_images = base.to_dynamic_images(&self.limits)?.into_owned();
        let (base_w, base_h) = base_dyn_images.maybe_first()?.0.dimensions();

        let overlay_dyn_images = overlay.to_dynamic_images(&self.limits)?;

        base_dyn_images.iter_images_mut(|f, i| {
            let current_overlay = &overlay_dyn_images.get_circular_neg(i as isize).0;
            let current_overlay = current_overlay.resize_exact(base_w, base_h, FilterType::Nearest);
            let mut original_fb = FrameBufferOwned::new_from_dyn_image(f);
            let mut fb = FrameBufferOwned::new_from_dyn_image(&current_overlay);
            ops::filter::opacity(fb.fb_mut(), 0.45);

            ops::overlay::blend(original_fb.fb_mut(), fb.fb(), 0, 0);
            original_fb.into_dyn_image()
        });

        Ok(MediaObject::DynamicImages(base_dyn_images))
    }
}
