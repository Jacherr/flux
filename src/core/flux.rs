use std::env::Args;
use std::fs::{read, write};
use std::io::{stdin, Read};

use anyhow::Context;
use serde_json::to_string;
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
    /// Information about the next input has been requested. Exit.
    MediaInfo,
    /// Version information was printed.
    PrintVersion,
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
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::Operation(operation) => {
                // operation string also contains options in the form of "operation[x=1:y=2:z=whatever]"
                self.media_container.handle_operation(operation.clone())?;
                self.previous_action = Some(StepAction::OperationPerformed(operation));
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::OutputPath(output) => {
                debug!("Writing output to {output}");
                // todo: support encoding for format based on file extension, and stdout
                let encoded = self.media_container.encode_next()?;
                write(output, encoded)?;
                self.previous_action = Some(StepAction::OutputWritten);
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::ImagePageLimit(lim) => {
                self.media_container.limits.frame_limit = Some(lim);
                self.previous_action = Some(StepAction::MetaPropertySet("page-limit"));
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::InputResolutionLimit((w, h)) => {
                self.media_container.limits.resolution_limit = Some((w, h));
                self.previous_action = Some(StepAction::MetaPropertySet("resolution-limit"));
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::VideoSupportDisabled => {
                self.media_container.limits.video_decode_permitted = false;
                self.previous_action = Some(StepAction::MetaPropertySet("video-decode-disabled"));
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::Info => {
                let info = self.media_container.info()?;
                let json = to_string(&info).context("Failed to serialize info output")?;
                println!("{json}");
                self.previous_action = Some(StepAction::MediaInfo);
                self.args_handler.set_version_flag_valid(false);
            },
            ArgType::Version => {
                if !self.args_handler.version_flag_valid() {
                    return Err(FluxError::Args(super::error::ArgError::UnrecognisedFlag(
                        "version".to_string(),
                    )));
                }

                let git_hash = option_env!("FLUX_GIT_HASH").unwrap_or("Unknown");
                let version = option_env!("FLUX_VERSION").unwrap_or("Unknown");

                println!("flux {version} (commit {git_hash})");
                self.previous_action = Some(StepAction::PrintVersion)
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
