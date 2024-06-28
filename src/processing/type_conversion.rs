use std::time::Duration;

use image::{DynamicImage, Frame};

use crate::util::convert_ratio_to_integer;

use super::dynamic_image_wrapper::DynamicImageWrapper;

pub fn convert_frames_to_dynamic_images(frames: Vec<Frame>) -> Vec<DynamicImageWrapper> {
    let mut images: Vec<DynamicImageWrapper> = vec![];

    for frame in frames {
        let dyn_image = DynamicImage::ImageRgba8(frame.clone().into_buffer());
        let (numer, denom) = frame.delay().numer_denom_ms();
        let delay = convert_ratio_to_integer(numer, denom) as u64;
        let image = DynamicImageWrapper::new(dyn_image, Some(Duration::from_millis(delay)));
        images.push(image);
    }

    images
}
