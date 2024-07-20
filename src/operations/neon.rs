use image::DynamicImage;

use crate::core::media_container::MediaContainer;
use crate::processing::gegl::softglow;
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn neon(&self) -> OperationResult {
        let input = self.pop_input()?;

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();

        dyn_images.iter_images_mut(|f, _| {
            let edges = imageproc::edges::canny(&f.to_luma8(), 50.0, 100.0);
            let more_edges = imageproc::edges::canny(&f.to_luma8(), 25.0, 50.0);
            let mut t = f.to_rgba16();
            for (x, y, px) in t.enumerate_pixels_mut() {
                if edges.get_pixel(x, y).0[0] == 0xff {
                    px.0 = [px.0[0], px.0[1], px.0[2], 0xffff];
                } else if more_edges.get_pixel(x, y).0[0] == 0xff {
                    px.0 = [px.0[0] / 2, px.0[1] / 2, px.0[2] / 2, 0xffff];
                } else {
                    px.0 = [0, 0, 0, 0xffff]
                }
            }

            let img = DynamicImage::ImageRgba8(DynamicImage::ImageRgba16(t).to_rgba8().into());

            softglow(&img, 1, 10, 45)
        });

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
