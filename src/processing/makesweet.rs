use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;

use crate::core::error::FluxError;

struct MakesweetDirectoryDeletionDefer<'a>(&'a str);
impl<'a> Drop for MakesweetDirectoryDeletionDefer<'a> {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub fn run_makesweet(images: &[&[u8]], operation: &str) -> Result<Vec<u8>, FluxError> {
    let id = std::process::id();

    let relative_dir = format!(
        "{id}-{}_makesweet",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    );

    let dir = format!("/tmp/{relative_dir}");

    fs::create_dir(&dir).context("Failed to crate temporary Makesweet directory")?;
    let _defer = MakesweetDirectoryDeletionDefer(&dir);

    // copy template to temp directory
    let template_location = format!("{dir}/{operation}.zip");
    fs::copy(format!("./makesweet/templates/{operation}.zip"), &template_location)
        .context("Failed to copy template")?;

    let mut file_names = vec![];
    for (n, image) in images.iter().enumerate() {
        let path = format!("{dir}/{n}.jpg");
        fs::write(&path, image).context("Failed to write Makesweet input")?;
        file_names.push(format!("{n}.jpg"));
    }

    let mut command = Command::new("docker");
    command.current_dir("./makesweet");

    let share_path = format!("{dir}:/share");
    command.args(&["run", "--rm", "-v", &share_path, "paulfitz/makesweet"]);

    let template = format!("{operation}.zip");
    command.args(&["--zip", &template]);
    command.arg("--in");

    for path in &file_names {
        command.arg(path);
    }

    let output_path = "o.gif";
    command.args(&["--gif", output_path]);

    let output = command
        .output()
        .map_err(|e| FluxError::ScriptError(format!("Failed to execute Makesweet: {e}")))?;

    if output.status.success() {
        let output = fs::read(format!("{dir}/{output_path}")).context("Failed to read Makesweet output")?;
        Ok(output)
    } else {
        Err(FluxError::ScriptError(format!(
            "Error performing Makesweet operation: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

/// Image must be of format JPEG
pub fn back_tattoo(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "back-tattoo")
}

/// Image must be of format JPEG
pub fn billboard_cityscape(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "billboard-cityscape")
}

/// Image must be of format JPEG
pub fn book(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "book")
}

/// Image must be of format JPEG
pub fn circuitboard(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "circuitboard")
}

/// Image must be of format JPEG
pub fn fortune_cookie(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "fortune-cookie")
}

/// Image must be of format JPEG
pub fn heart_locket(image1: &[u8], image2: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image1, image2], "heart-locket")
}

/// Image must be of format JPEG
pub fn flag(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "flag")
}

/// Image must be of format JPEG
pub fn flag2(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "flag2")
}

/// Image must be of format JPEG
pub fn rubiks(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "rubiks")
}

/// Image must be of format JPEG
pub fn toaster(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "toaster")
}

/// Image must be of format JPEG
pub fn valentine(image: &[u8]) -> Result<Vec<u8>, FluxError> {
    run_makesweet(&[image], "valentine")
}
