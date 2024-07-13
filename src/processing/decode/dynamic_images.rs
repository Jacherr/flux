use std::io::Cursor;

use image::codecs::gif::{GifDecoder, Repeat};
use image::codecs::png::PngDecoder;
use image::imageops::FilterType;
use image::{load_from_memory, AnimationDecoder, Frame, ImageResult};

use crate::core::error::FluxError;
use crate::core::media_container::DecodeLimits;
use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;
use crate::processing::ffmpeg;
use crate::processing::filetype::{get_sig_incl_mp4, Type};
use crate::processing::gif::gif_get_repeat_count;
use crate::processing::media_object::DynamicImagesMediaObject;
use crate::processing::type_conversion::convert_frames_to_dynamic_images;
use crate::vips::vips_transcode_to;

pub fn decode_to_dynamic_images(input: &[u8], limits: &DecodeLimits) -> Result<DynamicImagesMediaObject, FluxError> {
    let filetype = get_sig_incl_mp4(input).ok_or(FluxError::UnsupportedFiletype)?;

    let mut dyn_images = match filetype {
        Type::Jpeg => DynamicImagesMediaObject {
            images: vec![DynamicImageWrapper::new_static(load_from_memory(input)?)],
            audio: None,
            repeat: Repeat::Infinite,
        },
        Type::Png => decode_png_to_dynamic_images(input)?,
        Type::Webp => decode_webp_to_dynamic_images(input)?,
        Type::Gif => decode_gif_to_dynamic_images(input, limits.frame_limit)?,
        Type::Webm | Type::Mp4 => {
            if !limits.video_decode_permitted {
                return Err(FluxError::VideoDecodeDisabled);
            }
            decode_video_to_dynamic_images(input, limits)?
        },
    };

    // resize to fit any limits
    if let Some((w, h)) = limits.resolution_limit {
        let first = &dyn_images.images.first().unwrap().0;
        let (old_w, old_h) = (first.width(), first.height());

        if old_w as u64 > w && old_h as u64 > h {
            if old_w > old_h {
                let diff = old_w - old_h;
                dyn_images.iter_images_mut(|i, _| i.resize(w as u32 + diff, h as u32, FilterType::Nearest));
            } else {
                let diff = old_h - old_w;
                dyn_images.iter_images_mut(|i, _| i.resize(w as u32, h as u32 + diff, FilterType::Nearest));
            }
        }
    }

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

pub fn decode_webp_to_dynamic_images(buf: &[u8]) -> Result<DynamicImagesMediaObject, FluxError> {
    let buf = vips_transcode_to(buf, ".png")?;
    let image = DynamicImageWrapper::new(load_from_memory(&buf)?, None);
    Ok(DynamicImagesMediaObject {
        images: vec![image],
        repeat: Repeat::Infinite,
        audio: None,
    })
}

pub fn decode_gif_to_dynamic_images(
    buf: &[u8],
    frame_limit: Option<u64>,
) -> Result<DynamicImagesMediaObject, FluxError> {
    let decoder = GifDecoder::new(Cursor::new(buf))?;
    let frames = decoder
        .into_frames()
        .take(if let Some(f) = frame_limit {
            f as usize
        } else {
            usize::MAX
        })
        .collect::<ImageResult<Vec<Frame>>>()?;

    let repeats = gif_get_repeat_count(buf);

    let images = convert_frames_to_dynamic_images(frames);

    Ok(DynamicImagesMediaObject {
        images,
        repeat: repeats,
        audio: None,
    })
}

pub fn decode_video_to_dynamic_images(
    buf: &[u8],
    limits: &DecodeLimits,
) -> Result<DynamicImagesMediaObject, FluxError> {
    if !limits.video_decode_permitted {
        return Err(FluxError::VideoDecodeDisabled);
    }

    let split = ffmpeg::split_video(buf, limits.clone())?;

    let object = DynamicImagesMediaObject {
        images: split
            .0
            .into_iter()
            .map(|x| DynamicImageWrapper::new(x, None))
            .collect::<Vec<_>>(),
        audio: Some(split.1),
        repeat: Repeat::Infinite,
    };

    Ok(object)
}
