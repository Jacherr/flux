use crate::core::media_container::MediaContainer;
use crate::processing::gegl;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

pub struct BloomOptions {
    pub radius: Option<u64>,
    pub brightness: Option<u64>,
    pub sharpness: Option<u64>,
}

impl MediaContainer {
    pub fn bloom(&self, options: BloomOptions) -> OperationResult {
        let input = self.pop_input()?;
        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            gegl::softglow(
                f,
                options.radius.unwrap_or(5) as usize,
                options.brightness.unwrap_or(35) as usize,
                options.sharpness.unwrap_or(85) as usize,
            )
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
