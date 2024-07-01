use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::util::hash_buffer;
use crate::util::owned_child::IntoOwnedChild;

fn gegl(image: &DynamicImage, args: &[&str]) -> DynamicImage {
    let mut frame: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut frame), ImageFormat::Png).unwrap();
    let hash = hash_buffer(&frame);
    let path = format!("/tmp/gegl-{}.png", hash);
    std::fs::write(&path, frame).unwrap();

    let args = args.to_vec();

    let gegl_process = std::process::Command::new("gegl")
        .arg("-i")
        .arg(&path)
        .arg("-o")
        .arg(&path)
        .arg("--")
        .args(args)
        .spawn()
        .unwrap()
        .into_owned_child();

    let output = gegl_process.wait_with_output().unwrap();
    let file = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();

    if !output.stderr.is_empty() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    image::load_from_memory(&file).unwrap()
}

pub fn zoom_blur(image: &DynamicImage, factor: f64) -> DynamicImage {
    gegl(
        image,
        &[
            "motion-blur-zoom",
            &format!("factor={}", (factor / 10.0).clamp(0.0, 10.0)),
        ],
    )
}

pub fn softglow(image: &DynamicImage, radius: usize, brightness: usize, sharpness: usize) -> DynamicImage {
    gegl(
        image,
        &[
            "softglow",
            &format!("glow-radius={}", (radius as f64).clamp(1.0, 50.0)),
            &format!("brightness={}", (brightness as f64 / 100.0).clamp(0.0, 1.0)),
            &format!("sharpness={}", (sharpness as f64 / 100.0).clamp(0.0, 1.0)),
        ],
    )
}

pub fn fisheye(image: &DynamicImage) -> DynamicImage {
    gegl(
        image,
        &["lens-distortion", "main=95", "edge=100", "zoom=100", "brighten=20"],
    )
}

pub fn paint(image: &DynamicImage) -> DynamicImage {
    gegl(image, &["waterpixels"])
}

pub fn neon(image: &DynamicImage, factor: f64) -> DynamicImage {
    gegl(image, &["edge-neon", &format!("radius={}", factor * 2.0)])
}

pub fn globe(image: &DynamicImage) -> DynamicImage {
    gegl(image, &["apply-lens", "refraction-index=2.5"])
}
