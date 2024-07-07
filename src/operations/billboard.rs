use image::ImageFormat;

use crate::core::media_container::MediaContainer;
use crate::processing::makesweet::billboard_cityscape;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn billboard(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = billboard_cityscape(&enc)?;

        Ok(MediaObject::Encoded(result))
    }
}
