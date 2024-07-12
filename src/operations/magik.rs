use std::ptr;

use image::{DynamicImage, GenericImage, Rgba};
use lqr::lqr_carver::{
    lqr_carver_destroy, lqr_carver_flatten, lqr_carver_init, lqr_carver_new, lqr_carver_resize, lqr_carver_scan,
    lqr_carver_scan_reset, lqr_carver_set_preserve_input_image,
};

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

fn carve(image: &DynamicImage, x: i32, y: i32) -> DynamicImage {
    let width = image.width();
    let height = image.height();

    let raw = image.to_rgba8();
    let raw = raw.as_raw().as_ptr();

    unsafe {
        let carver = lqr_carver_new(raw as *mut u8, image.width() as _, image.height() as _, 4);
        lqr_carver_init(carver, 1, 0.0);
        lqr_carver_set_preserve_input_image(carver);
        lqr_carver_resize(carver, x, y);
        lqr_carver_flatten(carver);
        lqr_carver_resize(carver, width as i32, height as i32);
        lqr_carver_scan_reset(carver);
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut rgba: *mut u8 = ptr::null_mut();
        let mut img = DynamicImage::new_rgba8(width, height);
        while lqr_carver_scan(carver, &mut x, &mut y, &mut rgba) != 0 {
            img.unsafe_put_pixel(x as _, y as _, Rgba([*rgba, *rgba.add(1), *rgba.add(2), *rgba.add(3)]));
        }
        lqr_carver_destroy(carver);
        img
    }
}

impl MediaContainer {
    pub fn magik(&self) -> OperationResult {
        let input = self.pop_input()?;
        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| carve(f, f.width() as i32 / 2, f.height() as i32 / 2));

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
