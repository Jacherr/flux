use std::env::Args;
use std::fs::{read, write};
use std::io::{stdin, Read};

use tracing::debug;

use crate::core::args::ArgType;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::args::ArgsHandler;
use super::error::FluxError;

#[derive(PartialEq, Clone)]
pub enum StepAction {
    /// An input was added to the input queue.
    InputConsumed,
    /// An operation was performed on all or some of the current inputs.
    OperationPerformed(String),
    /// The final output file descriptor was written to. This terminates this Flux instance.
    OutputWritten,
    /// Some meta proprty has been set.
    MetaPropertySet(&'static str),
}

/// Main stateful struct for the current Flux instance.
///
/// A Flux instance must be stepped through to perform any action.
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

    /// Validates arguments before processing, to ensure that there was nothing weird supplied.
    pub fn validate_args(&self) -> Result<(), FluxError> {
        let _args = self.args_handler.fork();

        Ok(())
    }

    /// Steps through this instance by consuming the next input argument(s) and actioning upon it.
    pub fn step(&mut self) -> Result<StepAction, FluxError> {
        // if we already wrote the output theres nothing left to do
        if self.previous_action == Some(StepAction::OutputWritten) {
            return Err(FluxError::NothingToDo);
        }

        let next_arg = self.args_handler.parse_next().map_err(|e| FluxError::Args(e))?;

        match next_arg {
            ArgType::InputPath(input) => {
                debug!("Reading input {input} to queue");
                let data = self.read_input(&input)?;
                self.media_container.push_input(MediaObject::Encoded(data));
                self.previous_action = Some(StepAction::InputConsumed);
            },
            ArgType::Operation(operation) => {
                // operation string also contains options in the form of "operation[x=1:y=2:z=whatever]"
                self.media_container.handle_operation(operation.clone())?;
                self.previous_action = Some(StepAction::OperationPerformed(operation));
            },
            ArgType::OutputPath(output) => {
                debug!("Writing output to {output}");
                // todo: support encoding for format based on file extension, and stdout
                let encoded = self.media_container.encode_next()?;
                write(output, encoded)?;
                self.previous_action = Some(StepAction::OutputWritten);
            },
        }

        Ok(self.previous_action.clone().unwrap())
    }

    /// Reads an input file via filename or stdin.
    fn read_input(&self, path: &str) -> Result<Vec<u8>, FluxError> {
        if path == "STDIN" {
            let mut buf = vec![];
            stdin().read_to_end(&mut buf)?;
            Ok(buf)
        } else {
            Ok(read(path)?)
        }
    }
}
