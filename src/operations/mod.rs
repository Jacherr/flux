use crate::core::error::FluxError;
use crate::processing::media_object::MediaObject;

pub mod blur;
pub mod reverse;

pub type OperationResult = Result<MediaObject, FluxError>;
