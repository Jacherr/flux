use std::collections::HashMap;

use crate::core::error::{ArgError, FluxError};
use crate::core::input_queue::InputQueue;

use super::media_object::MediaObject;

/// Main media container for Flux. Contains everything needed to process a range of input formats by
/// splitting it down to its base components.\ These base components are typically a 2D Vec of
/// images (derived from input frames) - one Vec for each current input, and any audio data in the
/// event of a video input. The internal details are transparently handled. One operations require
/// certain data to be present (i.e., audio, or multiple frames).\
/// When an operation is completed, the result is pushed to the end of the input queue. \
/// **NOTE: This means that if there are excess inputs, those excess inputs will be consumed BEFORE
/// the result of the previous operation!**
pub struct MediaContainer {
    input_queue: InputQueue,
    pub frame_limit: Option<u64>,
}
impl MediaContainer {
    pub fn new() -> Self {
        Self {
            input_queue: InputQueue::new(),
            frame_limit: None,
        }
    }

    pub fn push_input(&self, input: MediaObject) {
        self.input_queue.push(input)
    }

    pub fn pop_input(&self) -> Result<MediaObject, FluxError> {
        self.input_queue
            .unshift()
            .ok_or(FluxError::Args(ArgError::ArgsExhausted))
    }

    /// Parse format: operation[x=1:y=2:z=whatever]
    fn parse_operation_name(operation: &str) -> Result<(String, HashMap<String, String>), FluxError> {
        if operation.contains("[") && operation.chars().next() != Some('[') {
            if !operation.ends_with("]") {
                return Err(FluxError::Args(ArgError::FlagOptionParseError(format!(
                    "Flag options missing termination for {}",
                    operation
                ))));
            }

            let options_start = operation.find("[").unwrap();
            let name = &operation[..options_start];
            let options = operation[options_start + 1..operation.len() - 1].split(":");

            let mut parsed_options = HashMap::new();
            for option in options {
                let split = option
                    .split_once("=")
                    .ok_or(FluxError::Args(ArgError::FlagOptionParseError(format!(
                        "Option \"{option}\" has a key, but no value"
                    ))))?;

                if split.0.is_empty() {
                    return Err(FluxError::Args(ArgError::FlagOptionParseError(
                        "Option key cannot be blank".to_owned(),
                    )));
                } else if split.1.is_empty() {
                    return Err(FluxError::Args(ArgError::FlagOptionParseError(format!(
                        "Option \"{option}\" has a key, but no value"
                    ))));
                }

                parsed_options.insert(split.0.to_owned(), split.1.to_owned());
            }

            Ok((name.to_owned(), parsed_options))
        } else {
            Ok((operation.to_owned(), HashMap::new()))
        }
    }

    /// Handles a new operation. When the operation (successfully) completes, the result of the
    /// operation will be pushed to the input queue.\
    /// To get the output of this operation, call `get_output`. This will pop the input queue and
    /// encode the data (if necessary).
    pub fn handle_operation(&self, operation: String) -> Result<(), FluxError> {
        let parsed = MediaContainer::parse_operation_name(&operation)?;

        let result = match &parsed.0[..] {
            "reverse" => self.reverse()?,
            _ => unimplemented!(),
        };

        self.push_input(result);
        Ok(())
    }

    pub fn encode_next(&self) -> Result<Vec<u8>, FluxError> {
        let next_image = self.pop_input()?;

        if self.input_queue.len() > 0 {
            return Err(FluxError::ResidualImages(self.input_queue.len() as u64));
        }

        next_image.encode()
    }
}
