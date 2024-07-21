use std::time::Duration;

use image::{DynamicImage, Rgba};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn spin(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::spin_video(v?).map(MediaObject::Encoded);
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        if dyn_images.images.len() == 1 {
            dyn_images.images.first_mut().unwrap().1 = Some(Duration::from_millis(50));
            dyn_images.images = dyn_images.images.into_iter().cycle().take(35).collect::<Vec<_>>();
        }

        let amt = dyn_images.images.len();

        dyn_images.iter_images_mut(|f, i| {
            DynamicImage::ImageRgba8(rotate_about_center(
                &f.to_rgba8(),
                ((i * 360 / amt) as f32).to_radians(),
                Interpolation::Bilinear,
                Rgba([0, 0, 0, 0]),
            ))
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
