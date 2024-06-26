use crate::core::input_queue::InputQueue;

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
}
impl MediaContainer {
    pub fn new() -> Self {
        Self {
            input_queue: InputQueue::new(),
        }
    }

    pub fn push_input(&self, input: Vec<u8>) {
        self.input_queue.push(input)
    }
}
