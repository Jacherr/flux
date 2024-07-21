use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;
use crate::processing::spread_image;

use super::OperationResult;

impl MediaContainer {
    pub fn spread(&self, strength: Option<u64>) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let strength = strength.unwrap_or(10);

        dyn_images.iter_images_mut(|f, _| {
            spread_image(f, strength as usize);
            f.clone()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
