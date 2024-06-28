use std::borrow::Cow;

use image::codecs::gif::Repeat;
use image::DynamicImage;

use crate::processing::decode::decode_to_dynamic_images;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::filetype::{get_sig_incl_mp4, Type};

use super::error::FluxError;

#[derive(Clone)]
pub struct DynamicImagesMediaObject {
    pub images: Vec<DynamicImageWrapper>,
    pub audio: Option<Vec<u8>>,
    pub repeat: Repeat,
}

pub enum MediaObject {
    Encoded(Vec<u8>),
    DynamicImages(DynamicImagesMediaObject),
}
impl MediaObject {
    pub fn to_dynamic_images(&self) -> Result<Cow<DynamicImagesMediaObject>, FluxError> {
        match self {
            Self::DynamicImages(x) => Ok(Cow::Borrowed(x)),
            Self::Encoded(e) => Ok(Cow::Owned(decode_to_dynamic_images(e)?)),
        }
    }

    pub fn is_encoded_video(&self) -> bool {
        match self {
            Self::DynamicImages(_) => false,
            Self::Encoded(enc) => {
                let ty = get_sig_incl_mp4(enc);
                ty.is_some_and(|ty| ty == Type::Mp4 || ty == Type::Webm)
            },
        }
    }

    pub fn unwrap_encoded(&self) -> &[u8] {
        if let Self::Encoded(x) = self { x } else { unreachable!() }
    }
}
