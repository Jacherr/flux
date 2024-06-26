use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgError {
    #[error("Unexpected EOF in arguments")]
    ArgsExhausted,
    #[error("Unrecognised flag: {0}")]
    UnrecognisedFlag(String),
}

#[derive(Error, Debug)]
pub enum FluxError {
    #[error("Failed to parse input arguments: {0}")]
    Args(ArgError),
    #[error("There are no more steps that this instance can take")]
    NothingToDo,
}
