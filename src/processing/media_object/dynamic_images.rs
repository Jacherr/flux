use std::io::Cursor;

use image::codecs::gif::Repeat;
use image::codecs::jpeg::JpegEncoder;
use image::{load_from_memory, DynamicImage, ExtendedColorType, ImageFormat};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::core::error::FluxError;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::util::collapse_neg;

#[derive(Clone)]
pub struct DynamicImagesMediaObject {
    pub images: Vec<DynamicImageWrapper>,
    pub audio: Option<Vec<u8>>,
    pub repeat: Repeat,
}
impl DynamicImagesMediaObject {
    pub fn iter_images_mut<T: Fn(&mut DynamicImage, usize) -> DynamicImage + Send + Sync>(
        &mut self,
        func: T,
    ) -> &mut Self {
        self.images
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, img)| img.0 = func(&mut img.0, i));

        self
    }

    pub fn iter_images_mut_fallible<
        T: Fn(&mut DynamicImage, usize) -> Result<DynamicImage, FluxError> + Send + Sync,
    >(
        &mut self,
        func: T,
    ) -> Result<&mut Self, FluxError> {
        self.images.par_iter_mut().enumerate().try_for_each(
            |(i, img): (usize, &mut DynamicImageWrapper)| match func(&mut img.0, i) {
                Ok(r) => {
                    img.0 = r;
                    return Ok(());
                },
                Err(e) => return Err(e),
            },
        )?;

        Ok(self)
    }

    pub fn update_quality(&mut self, quality: u8) -> Result<(), FluxError> {
        self.iter_images_mut_fallible(|image, _| {
            let mut buf = Vec::new();
            let mut encoder = JpegEncoder::new_with_quality(&mut buf, quality);
            encoder.encode(
                &image.to_rgb8().into_raw(),
                image.width(),
                image.height(),
                ExtendedColorType::Rgb8,
            )?;

            Ok(load_from_memory(&buf)?)
        })?;

        Ok(())
    }

    pub fn encode_as_png_frames(&self) -> Result<Vec<Vec<u8>>, FluxError> {
        let frames = self
            .images
            .par_iter()
            .map(|img| {
                let image = &img.0;

                let mut buf: Vec<u8> = Vec::with_capacity(1024usize.pow(2) * 25);
                if let Err(e) = image
                    .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
                    .map_err(|e| FluxError::ScriptError(e.to_string()))
                {
                    return Err(e);
                };

                Ok(buf)
            })
            .collect::<Vec<_>>();

        let mut out = Vec::new();
        for f in frames {
            out.push(f?);
        }

        Ok(out)
    }

    pub fn get_circular_neg(&self, index: isize) -> &DynamicImageWrapper {
        let corrected_index = collapse_neg(self.images.len() as isize, index);
        self.images.get(corrected_index).unwrap()
    }

    pub fn get_circular_neg_mut(&mut self, index: isize) -> &mut DynamicImageWrapper {
        let corrected_index = collapse_neg(self.images.len() as isize, index);
        self.images.get_mut(corrected_index).unwrap()
    }

    pub fn maybe_first(&self) -> Result<&DynamicImageWrapper, FluxError> {
        self.images
            .first()
            .ok_or(FluxError::CorruptInput("Input has no frames".to_owned()))
    }

    pub fn maybe_first_mut(&mut self) -> Result<&mut DynamicImageWrapper, FluxError> {
        self.images
            .first_mut()
            .ok_or(FluxError::CorruptInput("Input has no frames".to_owned()))
    }

    pub fn into_images(self) -> Vec<DynamicImageWrapper> {
        self.images
    }
}
