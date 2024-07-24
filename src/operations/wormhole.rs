use std::time::Duration;

use image::codecs::gif::Repeat;

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;
use crate::processing::{implode, swirl};
use crate::util::collapse;

use super::OperationResult;

impl MediaContainer {
    pub fn wormhole(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        let mut index = 0;
        if dyn_images.images.len() == 1 {
            let f = dyn_images.maybe_first_mut()?;
            f.1 = Some(Duration::from_millis(60));
        }

        // repeat gif until we hit the correct number of frames
        while dyn_images.images.len() < 50 {
            let new_image = dyn_images.images[collapse(dyn_images.images.len(), index)].clone();
            dyn_images.images.push(new_image);
            index += 1;
        }

        dyn_images.iter_images_mut(|image, index| {
            let s_arg = index as f32 / 10.0;
            let i_arg = index as f32 / 13f32;

            let s_img = unsafe { swirl(image, s_arg) };
            let i_img = implode(&s_img, i_arg);
            i_img.brighten((index * 6) as i32).into_rgb8().into()
        });

        dyn_images.repeat = Repeat::Finite(0);

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
