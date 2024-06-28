use std::io::Cursor;

use image::codecs::gif::Repeat;
use image::codecs::png::PngDecoder;
use image::{load_from_memory, AnimationDecoder};

use crate::core::error::FluxError;
use crate::core::media_object::DynamicImagesMediaObject;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::filetype::{get_sig_incl_mp4, Type};
use crate::processing::type_conversion::convert_frames_to_dynamic_images;

pub fn decode_to_dynamic_images(input: &[u8]) -> Result<DynamicImagesMediaObject, FluxError> {
    let filetype = get_sig_incl_mp4(input).ok_or(FluxError::UnsupportedFiletype)?;

    let dyn_images = match filetype {
        Type::Jpeg => DynamicImagesMediaObject {
            images: vec![DynamicImageWrapper::new_static(load_from_memory(input)?)],
            audio: None,
            repeat: Repeat::Infinite,
        },
        Type::Png => decode_png_to_dynamic_images(input)?,
        _ => todo!(),
    };

    Ok(dyn_images)
}

pub fn decode_png_to_dynamic_images(buf: &[u8]) -> Result<DynamicImagesMediaObject, FluxError> {
    let cursor = Cursor::new(buf);

    let decoder = PngDecoder::new(cursor)?;
    if decoder.is_apng()? {
        let apng_decoder = decoder.apng()?;
        let into_frames = apng_decoder.into_frames();
        let mut frames = vec![];
        for frame in into_frames {
            frames.push(frame?);
        }
        let images = convert_frames_to_dynamic_images(frames);

        Ok(DynamicImagesMediaObject {
            images,
            audio: None,
            repeat: Repeat::Infinite,
        })
    } else {
        let dyn_image = load_from_memory(buf)?;

        let image = DynamicImageWrapper::new(dyn_image, None);

        Ok(DynamicImagesMediaObject {
            images: vec![image],
            audio: None,
            repeat: Repeat::Infinite,
        })
    }
}
