use image::codecs::gif::Repeat;
use image::DynamicImage;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;

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
}
