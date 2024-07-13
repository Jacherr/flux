use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations::scramble_video;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn scramble(&self) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video() {
            let s = scramble_video(v)?;

            return Ok(MediaObject::Encoded(s));
        }

        unimplemented!()
    }
}
