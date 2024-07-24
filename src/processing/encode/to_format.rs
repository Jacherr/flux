use crate::core::error::FluxError;
use crate::processing::filetype::FileType;

pub fn video_transcode(buf: &[u8], to: FileType) -> Result<Vec<u8>, FluxError> {
    if to != FileType::Mp4 && to != FileType::Webm {
        return Err(FluxError::UnsupportedFiletype);
    }

    todo!()
}
