use crate::core::media_container::MediaContainer;
use crate::processing::gegl::zoom_blur;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn zoom_blur(&self, power: Option<f32>) -> OperationResult {
        let input = self.pop_input()?;
        let power = power.unwrap_or(2.0).clamp(-10.0, 10.0) / 10.0;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        dyn_images.iter_images_mut(|f, _| zoom_blur(f, power));

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
