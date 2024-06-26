#![allow(internal_features)]
#![feature(
    decl_macro,
    const_slice_from_raw_parts_mut,
    const_mut_refs,
    core_intrinsics,
    const_fn_floating_point_arithmetic
)]

use core::flux::Flux;

pub mod core;
pub mod processing;

fn main() {
    let args = std::env::args();
    let flux = Flux::new(args);
}
