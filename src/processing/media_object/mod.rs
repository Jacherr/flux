use std::borrow::Cow;

use crate::core::error::FluxError;
use crate::core::media_container::DecodeLimits;
use crate::processing::decode::dynamic_images::decode_to_dynamic_images;
use crate::processing::encode::encode_object;
use crate::processing::filetype::{get_sig_incl_mp4, Type};

pub mod dynamic_images;
pub use dynamic_images::DynamicImagesMediaObject;
use image::ImageFormat;

use super::encode::encode_first_frame_as;

pub enum MediaObject {
    Encoded(Vec<u8>),
    DynamicImages(DynamicImagesMediaObject),
}
impl MediaObject {
    pub fn to_dynamic_images(&self, limits: &DecodeLimits) -> Result<Cow<DynamicImagesMediaObject>, FluxError> {
        match self {
            Self::DynamicImages(x) => Ok(Cow::Borrowed(x)),
            Self::Encoded(e) => Ok(Cow::Owned(decode_to_dynamic_images(e, limits)?)),
        }
    }

    pub fn try_encoded_video(&self, decode_permitted: bool) -> Option<Result<&[u8], FluxError>> {
        match self {
            Self::DynamicImages(_) => None,
            Self::Encoded(enc) => {
                if self.is_encoded_video() {
                    if !decode_permitted {
                        return Some(Err(FluxError::VideoDecodeDisabled));
                    } else {
                        Some(Ok(enc))
                    }
                } else {
                    None
                }
            },
        }
    }

    pub fn is_encoded_video(&self) -> bool {
        match self {
            Self::DynamicImages(_) => false,
            Self::Encoded(enc) => {
                let ty = get_sig_incl_mp4(enc);
                if ty.is_some_and(|ty| ty == Type::Mp4 || ty == Type::Webm) {
                    true
                } else {
                    false
                }
            },
        }
    }

    pub fn unwrap_encoded(&self) -> &[u8] {
        if let Self::Encoded(x) = self { x } else { unreachable!() }
    }

    pub fn encode(self, limits: &DecodeLimits) -> Result<Vec<u8>, FluxError> {
        encode_object(self, None, limits)
    }

    pub fn encode_first_frame_as(self, format: ImageFormat, limits: &DecodeLimits) -> Result<Vec<u8>, FluxError> {
        encode_first_frame_as(self, format, limits)
    }
}
