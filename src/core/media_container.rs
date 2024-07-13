use std::time::{Duration, Instant};

use tracing::debug;

use crate::core::args::ArgsHandler;
use crate::core::error::{ArgError, FluxError};
use crate::core::input_queue::InputQueue;
use crate::processing::media_object::MediaObject;

#[derive(Default, Clone)]
pub struct DecodeLimits {
    pub frame_limit: Option<u64>,
    pub frame_rate_limit: Option<u64>,
    pub video_time_limit: Option<Duration>,
    pub resolution_limit: Option<(u64, u64)>,
    pub video_decode_permitted: bool,
}

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
    pub limits: DecodeLimits,
}
impl MediaContainer {
    pub fn new() -> Self {
        Self {
            input_queue: InputQueue::new(),
            limits: DecodeLimits {
                video_decode_permitted: true,
                ..Default::default()
            },
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

    /// Handles a new operation. When the operation (successfully) completes, the result of the
    /// operation will be pushed to the input queue.\
    /// To get the output of this operation, call `get_output`. This will pop the input queue and
    /// encode the data (if necessary).
    pub fn handle_operation(&self, operation: String) -> Result<(), FluxError> {
        let parsed = ArgsHandler::parse_operation_name(&operation)?;
        debug!("Performing operation {operation}");
        let start = Instant::now();

        let result = self.perform_operation(&parsed.0, parsed.1)?;

        debug!("Operation {operation}: took {:?}", start.elapsed());
        self.push_input(result);
        Ok(())
    }

    pub fn encode_next(&self) -> Result<Vec<u8>, FluxError> {
        let next_image = self.pop_input()?;

        if self.input_queue.len() > 0 {
            return Err(FluxError::ResidualImages(self.input_queue.len() as u64));
        }

        next_image.encode(&self.limits)
    }
}
