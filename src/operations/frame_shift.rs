use image::{DynamicImage, ImageBuffer, Rgba};

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::media_object::MediaObject;
use crate::util::collapse;

use super::OperationResult;

impl MediaContainer {
    pub fn frame_shift(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        if dyn_images.images.len() == 1 {
            return Err(FluxError::SinglePageMediaUnsupported);
        }

        let rgba8s = dyn_images
            .images
            .iter()
            .map(|i| i.0.clone().into_rgba8())
            .collect::<Vec<_>>();
        let len = rgba8s.len();

        let mut new_images: Vec<DynamicImageWrapper> = Vec::new();

        for (i, rgba8) in rgba8s.iter().enumerate() {
            let mut new_image = ImageBuffer::<Rgba<u8>, _>::new(rgba8.width(), rgba8.height());

            for (j, row) in new_image.enumerate_rows_mut() {
                let other_img_index = collapse(len, i + j as usize);
                let other_img = &rgba8s[other_img_index];
                let other_row = other_img.rows().nth(j as _).unwrap();

                for ((_, _, pixel), other_pixel) in row.zip(other_row) {
                    *pixel = *other_pixel;
                }
            }

            new_images.push(DynamicImageWrapper::new(
                DynamicImage::ImageRgba8(new_image),
                dyn_images.images.get(i).map(|i| i.1).flatten(),
            ));
        }

        dyn_images.images = new_images;

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
