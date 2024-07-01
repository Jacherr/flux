#![allow(internal_features)]
#![feature(
    decl_macro,
    const_slice_from_raw_parts_mut,
    const_mut_refs,
    core_intrinsics,
    const_fn_floating_point_arithmetic,
    try_trait_v2,
    let_chains,
    exact_size_is_empty
)]

use core::flux::{Flux, StepAction};
use std::process::ExitCode;

use anyhow::Context;
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
