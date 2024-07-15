use image::{GenericImage, Rgba};

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn posterize(&self, cols: &str) -> OperationResult {
        let c: Vec<u32> = cols.split(',').map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            let grey = f.to_luma8();
            for (x, y, px) in grey.enumerate_pixels() {
                let stage = (px.0[0] as f64 * c.len() as f64 / 255.0).floor() as usize;
                f.put_pixel(x, y, Rgba::from(c[stage].to_be_bytes()));
            }

            f.clone()
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
