use std::cell::RefCell;
use std::collections::HashMap;
use std::env::Args;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

use super::error::{ArgError, FluxError};

mod flag {
    use std::cell::LazyCell;
    use std::collections::HashMap;

    pub const FLAG_LH_PREFIX: &'static str = "--";
    pub const FLAG_SH_PREFIX: &'static str = "-";

    pub const FLAG_OPERATION: &'static str = "operation";
    pub const FLAG_INPUT: &'static str = "input";
    pub const FLAG_IMAGE_PAGE_LIMIT: &'static str = "page-limit";
    pub const FLAG_INPUT_RESOLUTION_LIMIT: &'static str = "res-limit";
    pub const FLAG_DISABLE_VIDEO_SUPPORT: &'static str = "disable-video-decode";
    pub const FLAG_IMAGE_INFO: &'static str = "info";
    pub const FLAG_VERSION: &'static str = "version";

    pub const FLAG_MAPPER: LazyCell<HashMap<&'static str, &'static str>> = LazyCell::new(|| {
        let mut h = HashMap::new();
        h.insert("o", FLAG_OPERATION);
        h.insert("i", FLAG_INPUT);
        h.insert("v", FLAG_VERSION);
        h
    });
}

/// A parsed argument type. When advancing the argument parser, it will return one of these types.
#[derive(Debug)]
pub enum ArgType {
    InputPath(String),
    Operation(String),
    OutputPath(String),
    ImagePageLimit(u64),
    InputResolutionLimit((u64, u64)),
    VideoSupportDisabled,
    Info,
    Version,
}

/// Internal metadata and stateful information used by the argument parser.
#[derive(Clone)]
struct ArgsMetaInternal {
    version_flag_valid: bool,
}
impl ArgsMetaInternal {
    pub fn new() -> Self {
        Self {
            version_flag_valid: true,
        }
    }
}

/// Accepts incoming std::env::Args and steps through them as needed.
pub struct ArgsHandler {
    args: Rc<RefCell<Args>>,
    meta: Rc<RefCell<ArgsMetaInternal>>,
}
impl ArgsHandler {
    pub fn new(args: Args) -> Self {
        let args = Rc::new(RefCell::new(args));

        // skip first arg
        let mut b = args.borrow_mut();
        let _ = b.next();
        drop(b);

        ArgsHandler {
            args,
            meta: Rc::new(RefCell::new(ArgsMetaInternal::new())),
        }
    }

    /// Parses the next argument. Consumes arguments as this function is called.
    pub fn parse_next(&self) -> Result<ArgType, ArgError> {
        let first = self.args.borrow_mut().next().ok_or(ArgError::ArgsExhausted)?;

        let flag_full_name = if first.starts_with(flag::FLAG_LH_PREFIX) {
            let flag_name = &first[flag::FLAG_LH_PREFIX.len()..];
            Some(flag_name)
        } else if first.starts_with(flag::FLAG_SH_PREFIX) {
            let flag_name = &first[flag::FLAG_SH_PREFIX.len()..];
            let mapped = flag::FLAG_MAPPER.get(&flag_name).map(|x| *x);
            mapped
        } else {
            None
        };

        // if not a flag, it should be an output path
        if let Some(name) = flag_full_name {
            self.handle_flag(name)
        } else {
            Ok(ArgType::OutputPath(first))
        }
    }

    /// Handles an incoming flag by name. May consume additional arguments in order to do this.
    fn handle_flag(&self, flag: &str) -> Result<ArgType, ArgError> {
        match flag {
            flag::FLAG_OPERATION => {
                let op = self.args.borrow_mut().next().ok_or(ArgError::ArgsExhausted)?;
                Ok(ArgType::Operation(op))
            },
            flag::FLAG_INPUT => {
                let input = self.args.borrow_mut().next().ok_or(ArgError::ArgsExhausted)?;
                Ok(ArgType::InputPath(input))
            },
            flag::FLAG_IMAGE_PAGE_LIMIT => {
                let limit = self.args.borrow_mut().next().ok_or(ArgError::ArgsExhausted)?;
                let limit = limit.parse::<u64>().map_err(|e| {
                    ArgError::FlagOptionParseError(format!("Invalid page cap value {limit}: {}", e.to_string()))
                })?;
                Ok(ArgType::ImagePageLimit(limit))
            },
            flag::FLAG_INPUT_RESOLUTION_LIMIT => {
                let limit = self.args.borrow_mut().next().ok_or(ArgError::ArgsExhausted)?;
                let split = limit.split("x").collect::<Vec<&str>>();
                let width = split
                    .get(0)
                    .ok_or(ArgError::FlagOptionParseError(format!(
                        "Invalid argument for resolution limit: missing width"
                    )))?
                    .parse::<u64>()
                    .map_err(|e| {
                        ArgError::FlagOptionParseError(format!(
                            "Invalid argument for resolution limit: Invalid width: {}",
                            e.to_string()
                        ))
                    })?;

                let height = split
                    .get(1)
                    .ok_or(ArgError::FlagOptionParseError(format!(
                        "Invalid argument for resolution limit: missing height"
                    )))?
                    .parse::<u64>()
                    .map_err(|e| {
                        ArgError::FlagOptionParseError(format!(
                            "Invalid argument for resolution limit: Invalid height: {}",
                            e.to_string()
                        ))
                    })?;

                Ok(ArgType::InputResolutionLimit((width, height)))
            },
            flag::FLAG_DISABLE_VIDEO_SUPPORT => Ok(ArgType::VideoSupportDisabled),
            flag::FLAG_IMAGE_INFO => Ok(ArgType::Info),
            flag::FLAG_VERSION => Ok(ArgType::Version),
            _ => Err(ArgError::UnrecognisedFlag(flag.to_owned())),
        }
    }

    /// Produces a clone of this ArgsHandler, allowing for lookahead validation etc.
    pub fn fork(&self) -> Self {
        Self {
            args: self.args.clone(),
            meta: self.meta.clone(),
        }
    }

    /// Parse format: operation[x=1:y=2:z=whatever]
    pub fn parse_operation_name(operation: &str) -> Result<(String, HashMap<String, String>), FluxError> {
        if operation.contains("[") && operation.chars().last() != Some('[') {
            if !operation.ends_with("]") {
                return Err(FluxError::Args(ArgError::FlagOptionParseError(format!(
                    "Flag options missing termination for {}",
                    operation
                ))));
            }

            let options_start = operation.find("[").unwrap();
            let name = &operation[..options_start];
            let options = OperationOptions::new(&operation[options_start + 1..operation.len() - 1]);

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

    pub fn set_version_flag_valid(&self, validity: bool) {
        self.meta.borrow_mut().version_flag_valid = validity;
    }

    pub fn version_flag_valid(&self) -> bool {
        self.meta.borrow().version_flag_valid
    }
}

struct OperationOptions<'a> {
    iter: Peekable<Chars<'a>>,
}
impl<'a> OperationOptions<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars().peekable(),
        }
    }
}
impl Iterator for OperationOptions<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.peek().is_none() {
            return None;
        }

        let mut current = String::new();

        while let Some(next) = self.iter.next() {
            if next == ';' {
                let prev = &current.chars().last();
                if prev == &Some('\\') {
                    current = current[..current.len() - 1].to_owned();
                } else {
                    break;
                }
            }

            current.push(next);
        }

        Some(current)
    }
}
