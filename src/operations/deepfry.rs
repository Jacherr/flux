use image::{GenericImage, GenericImageView, Pixel};

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn deepfry(&self) -> OperationResult {
        const T: u8 = 110;

        let image = self.pop_input()?;
        let mut image = image.to_dynamic_images(&self.limits)?.into_owned();

        image
            .iter_images_mut(|f, _| {
                for x in f.pixels().collect::<Vec<_>>() {
                    let mut rgba = x.2.to_rgba();
                    if rgba[0] < T {
                        rgba[0] = 0x0
                    } else {
                        rgba[0] = 0xff
                    };
                    if rgba[1] < T {
                        rgba[1] = 0x0
                    } else {
                        rgba[1] = 0xff
                    };
                    if rgba[2] < T {
                        rgba[2] = 0x0
                    } else {
                        rgba[2] = 0xff
                    };
                    f.put_pixel(x.0, x.1, rgba)
                }
                f.clone()
            })
            .update_quality(5)?;

        Ok(MediaObject::DynamicImages(image))
    }
}
