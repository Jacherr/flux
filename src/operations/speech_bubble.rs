use std::fs::read;
use std::time::Duration;

use anyhow::Context;
use image::imageops::{overlay, FilterType};
use image::{load_from_memory, Delay, DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn speech_bubble(&self, solid: bool) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let (w, h) = dyn_images.maybe_first()?.0.dimensions();

        let speechbubble_raw =
            read("./assets/image/speechbubble.png").context("Failed to load speechbubble template")?;
        let mut speechbubble = load_from_memory(&speechbubble_raw)?;
        speechbubble = speechbubble.resize_exact(w, h / 4, FilterType::Nearest);

        let mut canvas = DynamicImage::new_rgba8(w, h);

        overlay(&mut canvas, &speechbubble, 0, 0);

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                let px = canvas.get_pixel(x, y);
                if px == Rgba([0, 0, 0, 0]) {
                    unsafe {
                        canvas.unsafe_put_pixel(x, y, Rgba([127, 127, 127, 255]));
                    }
                } else {
                    let px = if solid {
                        Rgba([255, 255, 255, 255])
                    } else {
                        Rgba([0, 0, 0, 0])
                    };

                    unsafe {
                        canvas.unsafe_put_pixel(x, y, px);
                    }
                }
            }
        }

        dyn_images.iter_images_mut(|f, _| {
            let mut canvas = canvas.clone();
            for x in 0..canvas.width() {
                for y in 0..canvas.height() {
                    let px = canvas.get_pixel(x, y);
                    if px == Rgba([127, 127, 127, 255]) {
                        canvas.put_pixel(x, y, f.get_pixel(x, y));
                    }
                }
            }
            canvas
        });

        let (w, h) = dyn_images.maybe_first()?.0.dimensions();
        let repeat = dyn_images.repeat;

        let frames = dyn_images
            .into_images()
            .into_iter()
            .map(|x| (x.0, Delay::from_saturating_duration(x.1.unwrap_or(Duration::default()))))
            .collect::<Vec<_>>();

        let out = crate::processing::encode::gif::encode(frames, w as u16, h as u16, repeat)?;

        Ok(MediaObject::Encoded(out))
    }
}
