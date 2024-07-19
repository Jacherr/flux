use std::ffi::{c_int, CStr, CString};
use std::ptr::{null_mut, slice_from_raw_parts};

use ffi::*;
use image::{DynamicImage, ImageBuffer, Rgba};

use crate::core::error::FluxError;

pub mod ffi;

fn text_pango_safe(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('>', "&gt;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('%', "% ")
}

fn vips_get_error() -> String {
    let err = unsafe { v_get_error() };
    unsafe { CStr::from_ptr(err).to_str().unwrap().to_owned() }
}

pub fn vips_generate_caption(text: &str, width: usize) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let text = text_pango_safe(text);

    let mut buf = null_mut();
    let mut size: usize = 0;
    let mut height: usize = 0;
    let c_text = CString::new(text).map_err(|e| FluxError::ParameterError(e.to_string()))?;

    let res = unsafe { v_generate_caption_header(&mut buf, &mut size, &mut height, width, c_text.as_ptr()) };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error generating text: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer).unwrap();

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_generate_motivate_text(
    text: &str,
    colour: &str,
    width: usize,
    text_width: usize,
    pad_height: bool,
) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let text = format!(
        "<span foreground=\"{}\" background=\"black\"> {} </span>",
        colour,
        text_pango_safe(text)
    );

    let mut buf = null_mut();
    let mut size: usize = 0;
    let mut height: usize = 0;
    let c_text = CString::new(text).map_err(|e| FluxError::ParameterError(e.to_string()))?;

    let res = unsafe {
        v_generate_motivate_text(
            &mut buf,
            &mut size,
            &mut height,
            width,
            c_text.as_ptr(),
            text_width,
            pad_height as c_int,
        )
    };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error generating text: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer).unwrap();

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_generate_motivate_box_blank(
    width: usize,
    text_width: usize,
    pad_height: bool,
) -> Result<DynamicImage, FluxError> {
    // use tall character to maximise box height
    vips_generate_motivate_text("|", "black", width, text_width, pad_height)
}

pub fn vips_generate_meme_text(text: &str, width: usize, height: usize) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let real_text = format!("<span foreground=\"white\">{}</span>", text_pango_safe(text));

    let mut buf = null_mut();
    let mut size: usize = 0;

    let c_text = CString::new(real_text).map_err(|e| FluxError::ParameterError(e.to_string()))?;

    let res = unsafe { v_generate_meme_text(&mut buf, &mut size, height, width, c_text.as_ptr()) };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error generating text: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer).unwrap();

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_transcode_to(input: &[u8], format: &str) -> Result<Vec<u8>, FluxError> {
    let mut buf = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;
    let format = CString::new(format).unwrap();
    let res = unsafe { v_transcode_to(input.as_ptr(), input.len(), &mut buf, &mut size, format.as_ptr()) };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error transcoding: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    Ok(buffer)
}

pub fn vips_gravity(input: &[u8], width: usize, height: usize) -> Result<DynamicImage, FluxError> {
    let mut buf = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;
    let res = unsafe { v_gravity(input.as_ptr(), input.len(), &mut buf, &mut size, width, height) };

    if res != 0 {
        return Err(FluxError::ScriptError(format!("error resizing: {}", vips_get_error())));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer).unwrap();

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_generate_heart_locket_text(text: &str, width: usize, height: usize) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let text = format!(
        "<span foreground=\"black\" background=\"white\"> {} </span>",
        text_pango_safe(text)
    );

    let mut buf = null_mut();
    let mut size: usize = 0;
    let c_text = CString::new(text).map_err(|e| FluxError::ParameterError(e.to_string()))?;

    let res = unsafe { v_generate_heart_locket_text(&mut buf, &mut size, height, width, c_text.as_ptr()) };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error generating text: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer).unwrap();

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_canny(input: &[u8], width: usize, height: usize, sigma: f64) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let mut buf = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;
    let res = unsafe {
        v_canny(
            input.as_ptr(),
            input.len(),
            width as c_int,
            height as c_int,
            &mut buf,
            &mut size,
            sigma,
        )
    };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error performing edge detection: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer)
            .ok_or(FluxError::ScriptError("Failed to create image".to_owned()))?;

    Ok(DynamicImage::ImageRgba8(image))
}

pub fn vips_sobel(input: &[u8], width: usize, height: usize) -> Result<DynamicImage, FluxError> {
    unsafe { v_vips_init() };

    let mut buf = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;
    let res = unsafe {
        v_sobel(
            input.as_ptr(),
            input.len(),
            width as c_int,
            height as c_int,
            &mut buf,
            &mut size,
        )
    };

    if res != 0 {
        return Err(FluxError::ScriptError(format!(
            "error performing edge detection: {}",
            vips_get_error()
        )));
    }

    let buffer = unsafe { (*slice_from_raw_parts(buf, size)).to_owned() };
    unsafe { v_g_free(buf as *const ()) };

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer)
            .ok_or(FluxError::ScriptError("Failed to create image".to_owned()))?;

    Ok(DynamicImage::ImageRgba8(image))
}
