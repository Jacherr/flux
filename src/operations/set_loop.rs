use image::codecs::gif::Repeat;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn set_loop(&self, loops: i64) -> OperationResult {
        let input = self.pop_input()?;

        if input.is_encoded_video() {
            return Err(FluxError::InputMediaError(
                "Setting looping status is only supported on GIFs.".to_owned(),
            ));
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        const MAX: i64 = u16::MAX as i64;
        dyn_images.repeat = match loops {
            -1 => Repeat::Infinite,
            0..=MAX => Repeat::Finite(loops as u16),
            x => {
                return Err(FluxError::ParameterError(format!(
                    "Loop count {x} is out of range of -1 to {MAX} inclusive"
                )));
            },
        };

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
