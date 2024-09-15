#![allow(internal_features)]
#![feature(
    decl_macro,
    const_slice_from_raw_parts_mut,
    const_mut_refs,
    core_intrinsics,
    try_trait_v2,
    let_chains,
    exitcode_exit_method,
    compiler_builtins,
    portable_simd,
    link_llvm_intrinsics,
    simd_ffi
)]

use core::flux::{Flux, StepAction};
use std::process::{Command, ExitCode};
use std::{fs, thread};

use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use time::format_description;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::EnvFilter;
use vips::ffi::v_vips_init;

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

    unsafe {
        v_vips_init();
    }

    // handle SIGTERM for graceful shutdown of child processes (e.g. ffmpeg)
    let mut signals = Signals::new(&[SIGTERM]).unwrap();
    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGTERM {
                let current_pid = std::process::id();

                let mut cmd = Command::new("pkill");
                cmd.arg("-P");
                cmd.arg(&current_pid.to_string());
                let o = cmd.output();
                if let Err(e) = o {
                    eprintln!("flux: graceful shutdown failed: {}", e.to_string());
                    ExitCode::FAILURE.exit_process();
                };

                // cleanup all temp files
                let files = fs::read_dir("/tmp")
                    .map_err(|e| {
                        eprintln!("flux: graceful shutdown failed: {}", e.to_string());
                        ExitCode::FAILURE.exit_process();
                    })
                    .unwrap();

                for file in files {
                    if let Ok(f) = file {
                        let name = f.file_name();
                        let name = name.to_str().unwrap();
                        if name.starts_with(&format!("{current_pid}-")) {
                            let _ = fs::remove_file(format!("/tmp/{name}"));
                        }
                    }
                }

                ExitCode::SUCCESS.exit_process();
            }
        }
    });

    loop {
        let state = flux.step();

        match state {
            Ok(s) => {
                if s == StepAction::OutputWritten || s == StepAction::MediaInfo || s == StepAction::PrintVersion {
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
