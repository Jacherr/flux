use std::time::Duration;

use image::GenericImageView;

use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::{framebuffer, ops};
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;

use super::OperationResult;

impl MediaContainer {
    pub fn zoom(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        if dyn_images.images.len() == 1 {
            let first = dyn_images.maybe_first_mut()?;
            first.1 = Some(Duration::from_millis(20));
        };

        let mut new_images: Vec<DynamicImageWrapper> = Vec::new();
        let mut current: i32 = 1500;
        let mut image_index = 1;

        while current.is_positive() {
            let inner = dyn_images.get_circular_neg_mut(image_index);
            let (image, delay) = (&mut inner.0, inner.1);
            let (w, h) = image.dimensions();
            let (canvas_w, canvas_h) = (
                (w as f32 * (current as f32 / 100f32)).ceil() as u32,
                (h as f32 * (current as f32 / 100f32)).ceil() as u32,
            );

            let mut fb_canvas = framebuffer::new(canvas_w as _, canvas_h as _);
            let overlay_fb = FrameBufferOwned::new_from_dyn_image(&image);

            let mut ending = false;

            if canvas_w < w && canvas_h < h {
                ending = true;
                fb_canvas = ops::resize::nearest(&fb_canvas, w as _, h as _);
                ops::overlay::replace(&mut fb_canvas, &overlay_fb, 0, 0);
            } else {
                let (diff_w, diff_h) = (canvas_w - w, canvas_h - h);
                ops::overlay::replace(&mut fb_canvas, &overlay_fb, diff_w as isize / 2, diff_h as isize / 2);
            }

            fb_canvas = ops::resize::nearest(&fb_canvas, w as usize, h as usize);

            new_images.push(DynamicImageWrapper::new(
                framebuffer_to_dyn_image(fb_canvas.width as u32, fb_canvas.height as u32, fb_canvas.into_vec()),
                delay,
            ));

            if ending {
                break;
            };

            current -= 30;
            image_index += 1;
        }

        dyn_images.images = new_images;

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
