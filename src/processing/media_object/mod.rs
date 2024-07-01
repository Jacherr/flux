use std::borrow::Cow;

use crate::core::error::FluxError;
use crate::processing::decode::dynamic_images::decode_to_dynamic_images;
use crate::processing::encode::encode_object;
use crate::processing::filetype::{get_sig_incl_mp4, Type};

pub mod dynamic_images;
pub use dynamic_images::DynamicImagesMediaObject;

pub enum MediaObject {
    Encoded(Vec<u8>),
    DynamicImages(DynamicImagesMediaObject),
}
impl MediaObject {
    pub fn to_dynamic_images(&self, frame_limit: Option<u64>) -> Result<Cow<DynamicImagesMediaObject>, FluxError> {
        match self {
            Self::DynamicImages(x) => Ok(Cow::Borrowed(x)),
            Self::Encoded(e) => Ok(Cow::Owned(decode_to_dynamic_images(e, frame_limit)?)),
        }
    }

    pub fn try_encoded_video(&self) -> Option<&[u8]> {
        match self {
            Self::DynamicImages(_) => None,
            Self::Encoded(enc) => {
                let ty = get_sig_incl_mp4(enc);
                if ty.is_some_and(|ty| ty == Type::Mp4 || ty == Type::Webm) {
                    Some(enc)
                } else {
                    None
                }
            },
        }
    }

    pub fn unwrap_encoded(&self) -> &[u8] {
        if let Self::Encoded(x) = self { x } else { unreachable!() }
    }

    pub fn encode(self) -> Result<Vec<u8>, FluxError> {
        encode_object(self)
    }
}
