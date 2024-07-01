use core::slice;
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

    pub const FLAG_MAPPER: LazyCell<HashMap<&'static str, &'static str>> = LazyCell::new(|| {
        let mut h = HashMap::new();
        h.insert("o", "operation");
        h.insert("i", "input");
        h
    });
}

/// A parsed argument type. When advancing the argument parser, it will return one of these types.
#[derive(Debug)]
pub enum ArgType {
    InputPath(String),
    Operation(String),
    OutputPath(String),
}

/// Internal metadata and stateful information used by the argument parser.
#[derive(Clone)]
struct ArgsMetaInternal {}
impl ArgsMetaInternal {
    pub fn new() -> Self {
        Self {}
    }
}

/// Accepts incoming std::env::Args and steps through them as needed.
pub struct ArgsHandler {
    args: Rc<RefCell<Args>>,
    meta: ArgsMetaInternal,
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
            meta: ArgsMetaInternal::new(),
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

                println!("{}", split.1);
                parsed_options.insert(split.0.to_owned(), split.1.to_owned());
            }

            Ok((name.to_owned(), parsed_options))
        } else {
            Ok((operation.to_owned(), HashMap::new()))
        }
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
