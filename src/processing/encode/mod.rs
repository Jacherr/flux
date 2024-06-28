use std::time::Duration;

use image::codecs::png::PngEncoder;
use image::{Delay, ExtendedColorType, ImageEncoder};

use crate::core::error::FluxError;
use crate::core::media_object::MediaObject;
use crate::processing::ffmpeg::create_video_from_split;

pub mod gif;

pub fn encode_object(obj: MediaObject) -> Result<Vec<u8>, FluxError> {
    let encoded = match obj {
        MediaObject::DynamicImages(image_object) => {
            // we determine filetype to encode to either based on extension on filename provided, or
            // by taking a guess based on presence of multiple frames, audio, ...
            if let Some(ref audio) = image_object.audio {
                let inner_images = image_object.images.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
                create_video_from_split(inner_images, audio)
            } else if image_object.images.len() > 1 {
                let r#ref = image_object.images.first().unwrap();

                let frames = image_object
                    .images
                    .iter()
                    .map(|x| {
                        (
                            &x.0,
                            Delay::from_saturating_duration(x.1.unwrap_or(Duration::default())),
                        )
                    })
                    .collect::<Vec<_>>();

                super::encode::gif::encode(
                    frames,
                    r#ref.0.width() as u16,
                    r#ref.0.height() as u16,
                    image_object.repeat,
                )
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
