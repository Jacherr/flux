use std::io::{Cursor, Write};

use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn frames(&self) -> OperationResult {
        let input = self.pop_input()?;

        let dyn_images = input.to_dynamic_images(&self.limits)?;
        let frames = dyn_images.encode_as_png_frames()?;

        let mut buf = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(&mut buf);

        for (i, frame) in frames.iter().enumerate() {
            let name = format!("frame-{i}.png");
            zip.start_file(name, SimpleFileOptions::default())?;
            zip.write_all(frame)?;
        }

        let finished = zip.finish()?;
        let out = finished.clone().into_inner();

        Ok(MediaObject::Encoded(out))
    }
}
