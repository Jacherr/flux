use crate::processing::css_framebuffer::framebuffer;
use image::{DynamicImage, RgbImage, RgbaImage};
use std::ops::Deref;

pub struct FrameBufferOwned {
    _image: RgbaImage,
    fb: framebuffer,
}
impl FrameBufferOwned {
    pub fn new_from_imagebuffer_rgb(buf: RgbImage) -> Self {
        let buf = DynamicImage::ImageRgb8(buf).into_rgba8();
        let raw = buf.as_raw().as_ptr();
        let fb = framebuffer::from(buf.width() as usize, buf.height() as usize, raw as *mut u8);
        Self { _image: buf, fb }
    }

    pub fn new_from_dyn_image(f: &DynamicImage) -> Self {
        let buf = f.to_rgba8();
        let raw = buf.as_raw().as_ptr();

        let fb = framebuffer::from(buf.width() as usize, buf.height() as usize, raw as *mut u8);
        Self { _image: buf, fb }
    }

    pub fn into_dyn_image(self) -> DynamicImage {
        let (w, h, vec) = self.into_raw_parts();
        let imagebuffer = image::RgbaImage::from_vec(w, h, vec).unwrap();
        DynamicImage::ImageRgba8(imagebuffer)
    }

    pub fn fb(&self) -> &framebuffer {
        &self.fb
    }

    pub fn fb_mut(&mut self) -> &mut framebuffer {
        &mut self.fb
    }

    /// Converts this `FrameBufferOwned` to a `Vec<u8>`. The returned `Vec<u8>` reuses the same
    /// allocation as the initial image
    pub fn into_vec(self) -> Vec<u8> {
        // `framebuffer::vec()` reuses `self._image`'s buffer, so we need to make sure
        // not to run its destructor. 💥
        std::mem::forget(self._image);
        self.fb.into_vec()
    }

    /// Converts this `FrameBufferOwned` to a tuple containing its raw parts
    /// (width, height, vector)
    pub fn into_raw_parts(self) -> (u32, u32, Vec<u8>) {
        let w = self.width as u32;
        let h = self.height as u32;
        let v = self.into_vec();
        (w, h, v)
    }
}
impl Deref for FrameBufferOwned {
    type Target = framebuffer;
    fn deref(&self) -> &Self::Target {
        &self.fb
    }
}

unsafe impl Send for FrameBufferOwned {}
unsafe impl Sync for FrameBufferOwned {}
