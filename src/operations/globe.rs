use std::time::Duration;

use image::DynamicImage;

use crate::core::media_container::MediaContainer;
use crate::processing::gegl::globe;
use crate::processing::media_object::MediaObject;
use crate::processing::roll_image;

use super::OperationResult;

impl MediaContainer {
    pub fn globe(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let mut len = dyn_images.images.len();

        if len == 1 {
            dyn_images.images = dyn_images.images.into_iter().cycle().take(32).collect::<Vec<_>>();
            len = 32;
            dyn_images
                .images
                .iter_mut()
                .for_each(|i| i.1 = Some(Duration::from_millis(50)));
        }

        dyn_images.iter_images_mut(|f, i| {
            let roll_x = f.width() - ((f.width() / len as u32) * i as u32);

            let rolled = roll_image(f, roll_x, 0);
            DynamicImage::ImageRgba8(globe(&rolled).to_rgba8())
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
