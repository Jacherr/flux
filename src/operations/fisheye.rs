use crate::core::media_container::MediaContainer;
use crate::processing::gegl;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn fisheye(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| gegl::fisheye(f));

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
