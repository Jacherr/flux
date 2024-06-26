use std::env::Args;

use crate::processing::media_container::MediaContainer;

use super::args::ArgsHandler;
use super::error::FluxError;
use super::input_queue::InputQueue;

#[derive(PartialEq)]
pub enum StepAction {
    /// An input was added to the input queue.
    InputConsumed,
    /// An operation was performed on all or some of the current inputs.
    OperationPerformed(String),
    /// The final output file descriptor was written to. This terminates this Flux instance.
    OutputWritten,
}

/// Main stateful struct for the current Flux instance.
pub struct Flux {
    args_handler: ArgsHandler,
    previous_action: Option<StepAction>,
    media_container: MediaContainer,
}
impl Flux {
    pub fn new(args: Args) -> Self {
        Self {
            args_handler: ArgsHandler::new(args),
            previous_action: None,
            media_container: MediaContainer::new(),
        }
    }

    /// Steps through this instance by consuming the next input argument(s) and actioning upon it.
    pub fn step(&mut self) -> Result<StepAction, FluxError> {
        // if we already wrote the output theres nothing left to do
        if self.previous_action == Some(StepAction::OutputWritten) {
            return Err(FluxError::NothingToDo);
        }

        todo!()
    }
}
