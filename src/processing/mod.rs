use std::f32::consts::PI;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;
use std::simd::{f32x8, u32x8};

use image::imageops::overlay;
use image::{DynamicImage, GenericImage, GenericImageView};
use llvm_bindings::*;
use rand::Rng;

use crate::util::collapse_neg;

pub mod css_framebuffer;
pub mod decode;
pub mod dynamic_image_wrapper;
pub mod encode;
pub mod ffmpeg;
pub mod filetype;
pub mod framebuffer;
pub mod gegl;
pub mod gif;
pub mod makesweet;
pub mod media_object;
pub mod type_conversion;

pub fn roll_image(image: &DynamicImage, x_scroll: u32, y_scroll: u32) -> DynamicImage {
    let mut canvas = DynamicImage::new_rgba8(image.width(), image.height());
    let x_scroll = x_scroll % image.width();
    let y_scroll = y_scroll % image.height();
    let x_scroll_move_right = image.crop_imm(0, 0, x_scroll, image.height());
    let x_scroll_move_left = image.crop_imm(x_scroll, 0, image.width() - x_scroll, image.height());
    overlay(&mut canvas, &x_scroll_move_left, 0, 0);
    overlay(&mut canvas, &x_scroll_move_right, (image.width() - x_scroll) as i64, 0);
    let y_scroll_move_down = canvas.crop_imm(0, 0, image.width(), y_scroll);
    let y_scroll_move_up = canvas.crop_imm(0, y_scroll, image.width(), image.height() - y_scroll);
    overlay(&mut canvas, &y_scroll_move_up, 0, 0);
    overlay(&mut canvas, &y_scroll_move_down, 0, (image.height() - y_scroll) as i64);
    canvas
}

pub fn spread_image(image: &mut DynamicImage, spread_size: usize) {
    let w = image.width() as isize;
    let h = image.height() as isize;
    let spread_size = spread_size as isize;
    let mut rng = rand::thread_rng();
    for (x, y, px) in image.pixels().collect::<Vec<_>>() {
        let displacement_x = if spread_size == 0 {
            0
        } else {
            rng.gen_range(-spread_size..spread_size)
        };

        let displacement_y = if spread_size == 0 {
            0
        } else {
            rng.gen_range(-spread_size..spread_size)
        };

        let swap_x = collapse_neg(w - 1, x as isize + displacement_x) as u32;
        let swap_y = collapse_neg(h - 1, y as isize + displacement_y) as u32;
        let swap_px = image.get_pixel(swap_x, swap_y);
        image.put_pixel(swap_x, swap_y, px);
        image.put_pixel(x, y, swap_px);
    }
}

#[inline(always)]
fn reciprocal(x: f32) -> f32 {
    const EPSILON: f32 = 1e-12;

    let sign = x.signum();
    if sign * x > EPSILON { 1.0 / x } else { sign / EPSILON }
}

pub fn implode(img: &DynamicImage, amount: f32) -> DynamicImage {
    let width = img.width() as f32;
    let height = img.height() as f32;

    let mut scale_x = 1.0;
    let mut scale_y = 1.0;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let mut radius = center_x;

    if width > height {
        scale_y = width * reciprocal(height);
    } else if height > width {
        scale_x = height * reciprocal(width);
        radius = center_y;
    }

    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        let mut distance;
        let delta_y = scale_y * (y as f32 - center_y);
        for x in 0..img.width() {
            let delta_x = scale_x * (x as f32 - center_x);
            distance = delta_x * delta_x + delta_y * delta_y;
            if distance >= radius * radius {
                unsafe {
                    let px = img.unsafe_get_pixel(x, y);
                    out_img.unsafe_put_pixel(x, y, px);
                }
            } else {
                let mut factor = 1.0;
                if distance > 0.0 {
                    factor = (PI * distance.sqrt() * reciprocal(radius) / 2.0).sin().powf(-amount);
                }
                let mut new_x = factor * delta_x * reciprocal(scale_x) + center_x;
                let mut new_y = factor * delta_y * reciprocal(scale_y) + center_y;

                new_x = new_x.clamp(0.0, width - 1.0);
                new_y = new_y.clamp(0.0, height - 1.0);

                unsafe {
                    let px = img.unsafe_get_pixel(new_x as _, new_y as _);
                    out_img.unsafe_put_pixel(x, y, px);
                }
            }
        }
    }

    out_img
}

mod llvm_bindings {
    #![allow(improper_ctypes)]

    use core::simd::f32x8;

    extern "C" {
        #[link_name = "llvm.cos.v8f32"]
        pub fn cos_v8f32(i: f32x8) -> f32x8;
        #[link_name = "llvm.floor.v8f32"]
        pub fn floor_v8f32(i: f32x8) -> f32x8;
        #[link_name = "llvm.sin.v8f32"]
        pub fn sin_v8f32(i: f32x8) -> f32x8;
        #[link_name = "llvm.sqrt.v8f32"]
        pub fn sqrt_v8f32(i: f32x8) -> f32x8;
    }
}

#[target_feature(enable = "avx")]
pub unsafe fn swirl(img: &DynamicImage, intensity: f32) -> DynamicImage {
    let width = img.width();
    let height = img.height();

    let (mut x, mut y) = (0, 0);
    let mut out_img = DynamicImage::new_rgba8(width, height);

    for _ in (0..width * height)
        .step_by(8)
        .take_while(|i| i + 8 <= (width * height) - 1)
    {
        let xes = f32x8::from_array([
            x as f32,
            ((x + 1) % width) as f32,
            ((x + 2) % width) as f32,
            ((x + 3) % width) as f32,
            ((x + 4) % width) as f32,
            ((x + 5) % width) as f32,
            ((x + 6) % width) as f32,
            ((x + 7) % width) as f32,
        ]);
        let ys = f32x8::from_array([
            y as f32,
            (y + (x + 1) / width) as f32,
            (y + (x + 2) / width) as f32,
            (y + (x + 3) / width) as f32,
            (y + (x + 4) / width) as f32,
            (y + (x + 5) / width) as f32,
            (y + (x + 6) / width) as f32,
            (y + (x + 7) / width) as f32,
        ]);

        let (x_dists, y_dists) = (
            xes - f32x8::splat(width as f32) / f32x8::splat(2.0),
            ys - f32x8::splat(height as f32) / f32x8::splat(2.0),
        );

        let dists: f32x8 = sqrt_v8f32(x_dists * x_dists + y_dists * y_dists);

        let angles_in: f32x8 = dists / f32x8::splat(img.width() as f32) * f32x8::splat(2.0) * f32x8::splat(PI);
        let angles: f32x8 = sin_v8f32(angles_in) * f32x8::splat(intensity);

        let (s, c) = (sin_v8f32(angles), cos_v8f32(angles));

        let xes_in = x_dists * c // cosine
            - y_dists * s // sine
            + f32x8::splat((width + 1) as f32)
            / f32x8::splat(2.0);
        let ys_in = x_dists * s // sine
            + y_dists * c // cosine
            + f32x8::splat((height + 1) as f32)
            / f32x8::splat(2.0);

        let (xes_new, ys_new) = (floor_v8f32(xes_in).cast::<u32>(), floor_v8f32(ys_in).cast::<u32>());

        let maxx = u32x8::splat((width - 1) as u32);
        let maxy = u32x8::splat((height - 1) as u32);
        let min = u32x8::splat(0);
        let clamped_xes = xes_new
            .simd_lt(min)
            .select(min, xes_new)
            .simd_gt(maxx)
            .select(maxx, xes_new);

        let clamped_ys = ys_new
            .simd_lt(min)
            .select(min, ys_new)
            .simd_gt(maxy)
            .select(maxy, ys_new);

        let xes_arr = clamped_xes.as_array();
        let ys_arr = clamped_ys.as_array();

        for i in 0..8u32 {
            out_img.unsafe_put_pixel(
                (x + i) % (width),
                y + (x + i) / width,
                img.unsafe_get_pixel(xes_arr[i as usize] as _, ys_arr[i as usize] as _),
            );
        }

        (y, x) = (y + (x + 8) / width, (x + 8) % width);
    }

    if ((width * height) % 8) != 0 && (width - x) % 8 != 0 && y + 8 >= height - 1 {
        let remaining_steps = width - x;
        for i in 0..remaining_steps {
            let out = swirl_scalar(
                ((x + i) % width) as f32,
                (y + (x + i) / width) as f32,
                width,
                height,
                intensity,
            );
            let px = img.unsafe_get_pixel(out.0, out.1);
            out_img.unsafe_put_pixel((x + i) % width, y + (x + i) / width, px);
        }
    }

    out_img
}

#[inline(always)]
pub fn swirl_scalar(x: f32, y: f32, width: u32, height: u32, intensity: f32) -> (u32, u32) {
    let x_dist = x - width as f32 / 2.0;
    let y_dist = y - height as f32 / 2.0;

    let dist = (x_dist * x_dist + y_dist * y_dist).sqrt();

    let angle = (dist / width as f32 * 2.0 * PI).sin() * intensity;

    let (s, c) = angle.sin_cos();

    let x_new = (x_dist * c - y_dist * s + width as f32 / 2.0).floor() as u32;
    let y_new = (x_dist * s + y_dist * c + height as f32 / 2.0).floor() as u32;

    let clamped_x = x_new.clamp(0, width - 1);
    let clamped_y = y_new.clamp(0, height - 1);

    (clamped_x, clamped_y)
}
