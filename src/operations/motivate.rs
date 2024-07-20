use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use crate::core::media_container::MediaContainer;
use crate::processing::css_framebuffer::ops;
use crate::processing::framebuffer::FrameBufferOwned;
use crate::processing::media_object::MediaObject;
use crate::processing::type_conversion::framebuffer_to_dyn_image;
use crate::vips::{vips_generate_motivate_box_blank, vips_generate_motivate_text};

use super::OperationResult;

impl MediaContainer {
    pub fn motivate(&self, top: Option<String>, bottom: Option<String>) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let (in_width, in_height) = dyn_images.maybe_first()?.0.dimensions();

        let (extra_w, extra_h) = (100, 50);

        let mut top_img = DynamicImage::new_rgb8(in_width + extra_w, in_height + extra_h);
        for x in 0..top_img.width() {
            for y in 0..top_img.height() {
                unsafe { top_img.unsafe_put_pixel(x, y, Rgba([0, 0, 0, 255])) };
            }
        }

        dyn_images.iter_images_mut(|f, _| {
            let mut new = top_img.to_rgb8();
            let rect_x = ((extra_w / 2) - 5) as i32;
            let rect_y = ((extra_h / 2) - 5) as i32;
            let rect_white = Rect::at(rect_x, rect_y).of_size(in_width + 10, in_height + 10);
            draw_hollow_rect_mut(&mut new, rect_white, Rgb([255, 255, 255]));

            let mut fb = FrameBufferOwned::new_from_imagebuffer_rgb(new);
            let fb_overlay = FrameBufferOwned::new_from_dyn_image(f);
            ops::overlay::replace(fb.fb_mut(), fb_overlay.fb(), extra_w as isize / 2, extra_h as isize / 2);
            fb.into_dyn_image()
        });

        let top_text = if let Some(top) = top
            && !top.is_empty()
        {
            vips_generate_motivate_text(
                &top,
                "white",
                (in_width + extra_w) as usize,
                (in_height / 5) as usize,
                true,
            )?
        } else {
            vips_generate_motivate_box_blank((in_width + extra_w) as usize, (in_width / 5) as usize, false)?
        };

        let bottom_text = if let Some(t) = bottom
            && !t.is_empty()
        {
            Some(vips_generate_motivate_text(
                &t,
                "white",
                (in_width + extra_w) as usize,
                (in_height / 10) as usize,
                true,
            )?)
        } else {
            None
        };

        dyn_images.iter_images_mut(|f, _| {
            let mut canvas = unsafe {
                crate::processing::css_framebuffer::framebuffer::new_uninit(
                    (in_width + extra_w) as usize,
                    (in_height + extra_h + top_text.height() + bottom_text.as_ref().map(|b| b.height()).unwrap_or(0))
                        as usize,
                )
            };

            let fb = FrameBufferOwned::new_from_dyn_image(f);
            let top_text_fb = FrameBufferOwned::new_from_dyn_image(&top_text);

            ops::overlay::replace(&mut canvas, fb.fb(), 0, 0);
            ops::overlay::replace(&mut canvas, top_text_fb.fb(), 0, (in_height + extra_h) as isize);

            if let Some(ref b) = bottom_text {
                let bottom_text_fb = FrameBufferOwned::new_from_dyn_image(b);
                ops::overlay::replace(
                    &mut canvas,
                    bottom_text_fb.fb(),
                    0,
                    (in_height + extra_h + top_text.height()) as isize,
                );
            }

            framebuffer_to_dyn_image(canvas.width as u32, canvas.height as u32, canvas.into_vec())
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
