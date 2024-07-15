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
            for px in luma.pixels_mut() {
                px.0[0] = if px.0[0] < t { 0.0 } else { 1.0 }
            }

            luma.into()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }

    pub fn channels(&self, keep: Option<&str>) -> OperationResult {
        let v = keep.unwrap_or("rgb").to_ascii_lowercase();
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            let mut rgba16 = f.to_rgba16();
            for p in rgba16.pixels_mut() {
                if !v.contains('r') {
                    p.0[0] = 0;
                }

                if !v.contains('g') {
                    p.0[1] = 0;
                }

                if !v.contains('b') {
                    p.0[2] = 0;
                }
            }

            rgba16.into()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }

    pub fn edges(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| imageproc::edges::canny(&f.to_luma8(), 50.0, 100.0).into());

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
