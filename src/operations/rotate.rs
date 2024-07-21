use image::Rgba;
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn rotate(&self, deg: Option<u64>) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return ffmpeg_operations::rotate_video(v?, deg.unwrap_or(90) as usize).map(MediaObject::Encoded);
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        dyn_images.iter_images_mut(|f, _| {
            let deg = deg.unwrap_or(90);
            match deg {
                90 => f.rotate90(),
                180 => f.rotate180(),
                270 => f.rotate270(),
                deg => rotate_about_center(
                    &f.to_rgba8(),
                    (deg as f32).to_radians(),
                    Interpolation::Bicubic,
                    Rgba([0, 0, 0, 0]),
                )
                .into(),
            }
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
