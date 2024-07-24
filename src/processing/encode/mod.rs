use std::io::Cursor;
use std::time::Duration;

use image::codecs::png::PngEncoder;
use image::{load_from_memory, Delay, ExtendedColorType, GenericImageView, ImageEncoder, ImageFormat};

use crate::core::error::FluxError;
use crate::core::media_container::DecodeLimits;
use crate::processing::decode::dynamic_images::decode_to_dynamic_images;
use crate::processing::ffmpeg::{self, create_video_from_split};
use crate::processing::filetype::{get_sig_incl_mp4, Type};
use crate::processing::media_object::MediaObject;

pub mod gif;

pub fn encode_auto(obj: MediaObject, limits: &DecodeLimits) -> Result<Vec<u8>, FluxError> {
    let encoded = match obj {
        MediaObject::DynamicImages(image_object) => {
            // we determine filetype to encode to either based on extension on filename provided, or
            // by taking a guess based on presence of multiple frames, audio, ...
            if let Some(ref audio) = image_object.audio {
                let inner_images = image_object.images.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
                create_video_from_split(inner_images, audio, limits)
            } else if image_object.images.len() > 1 {
                let (w, h) = image_object.maybe_first()?.0.dimensions();
                let repeat = image_object.repeat;

                let frames = image_object
                    .into_images()
                    .into_iter()
                    .map(|x| (x.0, Delay::from_saturating_duration(x.1.unwrap_or(Duration::default()))))
                    .collect::<Vec<_>>();

                super::encode::gif::encode(frames, w as u16, h as u16, repeat)
            } else {
                let image = &image_object.images.first().unwrap().0;
                let rgba_image = image.to_rgba8();

                let mut buf: Vec<u8> = Vec::with_capacity(1024usize.pow(2) * 25);
                let encoder = PngEncoder::new(&mut buf);
                encoder.write_image(
                    &rgba_image.into_raw(),
                    image.width(),
                    image.height(),
                    ExtendedColorType::Rgba8,
                )?;

                Ok(buf)
            }
        },
        MediaObject::Encoded(enc) => Ok(enc),
    };

    Ok(encoded?)
}

pub fn encode_object(
    obj: MediaObject,
    format: Option<ImageFormat>,
    limits: &DecodeLimits,
) -> Result<Vec<u8>, FluxError> {
    if let Some(_f) = format {
        todo!()
    } else {
        encode_auto(obj, limits)
    }
}

pub fn encode_first_frame_as(
    obj: MediaObject,
    format: ImageFormat,
    limits: &DecodeLimits,
) -> Result<Vec<u8>, FluxError> {
    let encoded = match obj {
        MediaObject::DynamicImages(image_object) => {
            let frame_1 = image_object
                .images
                .first()
                .map(|x| &x.0)
                .ok_or(FluxError::Other("No images in sequence to encode".to_owned()))?;

            let mut out = Vec::new();

            if format == ImageFormat::Jpeg {
                let rgb8 = frame_1.to_rgb8();
                rgb8.write_to(&mut Cursor::new(&mut out), format)?;
            } else {
                frame_1.write_to(&mut Cursor::new(&mut out), format)?;
            }

            out
        },
        MediaObject::Encoded(enc) => {
            let enc_format = get_sig_incl_mp4(&enc).ok_or(FluxError::UnsupportedFiletype)?;
            match enc_format {
                Type::Jpeg | Type::Png | Type::Gif | Type::Webp => {
                    let dyn_images = decode_to_dynamic_images(&enc, limits)?;
                    let frame_1 = dyn_images
                        .images
                        .first()
                        .map(|x| &x.0)
                        .ok_or(FluxError::Other("No images in sequence to encode".to_owned()))?;

                    let mut out = Vec::new();

                    if format == ImageFormat::Jpeg {
                        let new = frame_1.to_rgb8();
                        new.write_to(&mut Cursor::new(&mut out), format)?;
                    } else {
                        frame_1.write_to(&mut Cursor::new(&mut out), format)?;
                    }

                    out
                },
                Type::Mp4 | Type::Webm => {
                    let first_frame_png = ffmpeg::get_video_first_frame(&enc)?;
                    let dyn_image = load_from_memory(&first_frame_png)?;
                    let mut out = Vec::new();
                    dyn_image.write_to(&mut Cursor::new(&mut out), format)?;

                    out
                },
            }
        },
    };

    Ok(encoded)
}
