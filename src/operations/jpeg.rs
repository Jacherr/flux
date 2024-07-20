use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::load_from_memory;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn jpeg(&self, quality: Option<u64>) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(q) = quality
            && (q > 100 || q < 1)
        {
            return Err(FluxError::ParameterError(
                "Quality must be between 1 and 100 inclusive".to_owned(),
            ));
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        dyn_images.iter_images_mut_fallible(|f, _| {
            let mut buf = Vec::new();
            let out = Cursor::new(&mut buf);
            let mut jpeg = JpegEncoder::new_with_quality(out, quality.map(|q| q as u8).unwrap_or(5));
            let rgb8 = f.to_rgb8();
            jpeg.encode_image(&rgb8)?;
            load_from_memory(&buf).map_err(|e| e.into())
        })?;

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
