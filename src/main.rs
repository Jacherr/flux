#![allow(internal_features)]
#![feature(
    decl_macro,
    const_slice_from_raw_parts_mut,
    const_mut_refs,
    core_intrinsics,
    const_fn_floating_point_arithmetic
)]

use core::flux::{Flux, StepAction};

use anyhow::Context;

pub mod core;
pub mod operations;
pub mod processing;
pub mod util;
pub mod vips;

fn main() {
    let args = std::env::args();
    let mut flux = Flux::new(args);

    flux.validate_args()
        .context("Failed to validate input arguments")
        .unwrap();

    loop {
        let state = flux.step().context("Failed to process step").unwrap();

        if state == StepAction::OutputWritten {
            break;
        }
    }
}
