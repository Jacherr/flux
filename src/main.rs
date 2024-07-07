#![allow(internal_features)]
#![feature(
    decl_macro,
    const_slice_from_raw_parts_mut,
    const_mut_refs,
    core_intrinsics,
    const_fn_floating_point_arithmetic,
    try_trait_v2,
    let_chains,
    exitcode_exit_method
)]

use core::flux::{Flux, StepAction};
use std::process::{Command, ExitCode};
use std::thread;

use anyhow::Context;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use time::format_description;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::EnvFilter;

pub mod core;
pub mod operations;
pub mod processing;
pub mod util;
pub mod vips;

fn main() -> ExitCode {
    let filter = EnvFilter::from_default_env();

    let description = "[year]-[month]-[day] [hour]:[minute]:[second]";

    tracing_subscriber::fmt()
        .with_timer(UtcTime::new(format_description::parse(description).unwrap()))
        .with_line_number(true)
        .with_env_filter(filter)
        .init();

    let args = std::env::args();
    let mut flux = Flux::new(args);

    // handle SIGTERM for graceful shutdown of child processes (e.g. ffmpeg)
    let mut signals = Signals::new(&[SIGTERM]).unwrap();
    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGTERM {
                let mut cmd = Command::new("pkill");
                cmd.arg("$$");
                let o = cmd.output();
                if let Err(e) = o {
                    eprintln!("flux: graceful shutdown failed: {}", e.to_string());
                    ExitCode::FAILURE.exit_process();
                }
                ExitCode::SUCCESS.exit_process();
            }
        }
    });

    loop {
        let state = flux.step().context("Failed to process step");

        match state {
            Ok(s) => {
                if s == StepAction::OutputWritten {
                    break ExitCode::SUCCESS;
                }
            },
            Err(e) => {
                eprintln!("{e:#}");
                break ExitCode::FAILURE;
            },
        }
    }
}
