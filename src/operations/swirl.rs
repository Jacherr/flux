use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;
use crate::processing::swirl;

use super::OperationResult;

impl MediaContainer {
    pub fn swirl(&self, strength: Option<f32>) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let strength = strength.unwrap_or(1.0);

        dyn_images.iter_images_mut(|f, _| unsafe { swirl(f, strength) });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
