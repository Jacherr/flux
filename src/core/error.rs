use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgError {
    #[error("Unexpected EOF in arguments")]
    ArgsExhausted,
    #[error("Unrecognised flag: {0}")]
    UnrecognisedFlag(String),
    #[error("Error parsing flag options: {0}")]
    FlagOptionParseError(String),
}

#[derive(Error, Debug)]
pub enum FluxError {
    #[error("Failed to parse input arguments: {0}")]
    Args(ArgError),
    #[error("There are no more steps that this instance can take")]
    NothingToDo,
    #[error("Error processing script: {0}\nThis is a bug. We would appreciate a report.")]
    ScriptError(String),
    #[error("Unsupported filetype")]
    UnsupportedFiletype,
    #[error("Input image is corrupt: {0}")]
    CorruptInput(String),
    #[error("Input parameter error: {0}")]
    ParameterError(String),
    #[error("{0} residual images after encoding")]
    ResidualImages(u64),
    #[error("Error with input image: {0}")]
    InputImageError(String),
    #[error("Error: {0}")]
    Other(String),
}

impl From<std::io::Error> for FluxError {
    fn from(value: std::io::Error) -> Self {
        Self::ScriptError(value.to_string())
    }
}

impl From<ImageError> for FluxError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::Decoding(e) => FluxError::CorruptInput(e.to_string()),
            otherwise => FluxError::ScriptError(otherwise.to_string()),
        }
    }
}
