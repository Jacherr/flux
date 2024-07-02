use crate::core::media_container::MediaContainer;
use crate::processing::gegl;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

pub struct BloomOptions {
    pub radius: Option<usize>,
    pub brightness: Option<usize>,
    pub sharpness: Option<usize>,
}

impl MediaContainer {
    pub fn bloom(&self, options: BloomOptions) -> OperationResult {
        let input = self.pop_input()?;
        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            gegl::softglow(
                f,
                options.radius.unwrap_or(5),
                options.brightness.unwrap_or(35),
                options.sharpness.unwrap_or(85),
            )
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
