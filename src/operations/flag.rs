use image::ImageFormat;

use crate::core::media_container::MediaContainer;
use crate::processing::makesweet::flag;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn flag(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = flag(&enc)?;

        Ok(MediaObject::Encoded(result))
    }
}
