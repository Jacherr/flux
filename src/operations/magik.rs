use std::ptr;
use std::time::Duration;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use lqr::lqr_carver::{
    lqr_carver_destroy, lqr_carver_flatten, lqr_carver_init, lqr_carver_new, lqr_carver_resize, lqr_carver_scan,
    lqr_carver_scan_reset, lqr_carver_set_preserve_input_image,
};

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;
use crate::util::windows2;

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

    pub fn gif_magik(&self) -> OperationResult {
        let input = self.pop_input()?;
        if input.is_encoded_video() {
            return Err(FluxError::InputMediaError(
                "Gif-magik is only supported on single-frame images. For other images try `magik` instead.".to_owned(),
            ));
        }

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        if dyn_images.images.len() > 1 {
            return Err(FluxError::InputMediaError(
                "Gif-magik is only supported on single-frame images. For other images try `magik` instead.".to_owned(),
            ));
        };

        dyn_images.images = dyn_images.images.into_iter().cycle().take(20).collect::<Vec<_>>();
        let (w, h) = dyn_images.images.first().unwrap().0.dimensions();

        windows2(&mut dyn_images.images, |first, second| {
            let new_image = {
                // `first` is prev
                let small_x = (w as f32 / 0.65).floor() as i32;
                let small_y = (h as f32 / 0.65).floor() as i32;
                carve(&first.0, small_x, small_y)
            };
            second.0 = new_image;
            second.1 = Some(Duration::from_millis(60));
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
