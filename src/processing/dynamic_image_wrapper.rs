use std::time::Duration;

use image::DynamicImage;

#[derive(Clone)]
pub struct DynamicImageWrapper(pub DynamicImage, pub Option<Duration>);
impl DynamicImageWrapper {
    pub fn new(image: DynamicImage, delay: Option<Duration>) -> Self {
        Self(image, delay)
    }

    pub fn new_static(image: DynamicImage) -> Self {
        Self(image, None)
    }
}
