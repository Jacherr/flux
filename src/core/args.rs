use std::cell::{LazyCell, RefCell};
use std::collections::HashMap;
use std::env::Args;
use std::rc::Rc;

use super::error::ArgError;

mod flag {
    use std::cell::LazyCell;
    use std::collections::HashMap;

    pub const FLAG_LH_PREFIX: &'static str = "--";
    pub const FLAG_SH_PREFIX: &'static str = "-";

    pub const FLAG_OPERATION: &'static str = "operation";
    pub const FLAG_INPUT: &'static str = "input";
    pub const FLAG_VERBOSE: &'static str = "verbose";

    pub const FLAG_MAPPER: LazyCell<HashMap<&'static str, &'static str>> = LazyCell::new(|| {
        let mut h = HashMap::new();
        h.insert("o", "operation");
        h.insert("i", "input");
        h.insert("v", "verbose");
        h
    });
}

/// A parsed argument type. When advancing the argument parser, it will return one of these types.
#[derive(Debug)]
pub enum ArgType {
    InputPath(String),
    Operation(String),
    Verbose,
    OutputPath(String),
}

/// Internal metadata and stateful information used by the argument parser.
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
            flag::FLAG_VERBOSE => Ok(ArgType::Verbose),
            _ => Err(ArgError::UnrecognisedFlag(flag.to_owned())),
        }
    }
}
