use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn threshold(&self, threshold: Option<f32>) -> OperationResult {
        let t = threshold.unwrap_or(0.5).clamp(0.0, 1.0);
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            let mut luma = f.to_luma_alpha32f();
            for (_, _, px) in luma.enumerate_pixels_mut() {
                px.0[0] = if px.0[0] < t { 0.0 } else { 1.0 }
            }

            luma.into()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
