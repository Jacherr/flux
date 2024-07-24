use crate::core::error::FluxError;
//use crate::framebuffer::FrameBuffer;
use crate::util::convert_ratio_to_integer;
use gif::{AnyExtension, Frame};
use image::codecs::gif::Repeat;
use image::{Delay, DynamicImage, GenericImageView};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::convert::TryInto;

fn convert_repeat(repeat: Repeat) -> gif::Repeat {
    match repeat {
        Repeat::Finite(a) => gif::Repeat::Finite(a),
        Repeat::Infinite => gif::Repeat::Infinite,
    }
}

pub fn encode(
    frames: Vec<(DynamicImage, Delay)>,
    width: u16,
    height: u16,
    repeat: Repeat,
) -> Result<Vec<u8>, FluxError> {
    let mut buf: Vec<u8> = vec![];

    if frames.is_empty() {
        return Ok(buf);
    };

    let mut encoder = gif::Encoder::new(&mut buf, width, height, &[]).map_err(|e| FluxError::Other(e.to_string()))?;

    encoder
        .set_repeat(convert_repeat(repeat))
        .map_err(|e| FluxError::Other(e.to_string()))?;

    let new_frames: Vec<gif::Frame> = frames
        .into_par_iter()
        .map(|(img, delay)| {
            let (w, h) = img.dimensions();
            let px = img.into_rgba8();
            let mut raw = px.into_raw();

            let mut frame = Frame::from_rgba_speed(w as _, h as _, &mut raw, 15);

            let (numer, denom) = delay.numer_denom_ms();
            let frame_delay = convert_ratio_to_integer(numer, denom);
            frame.delay = (frame_delay / 10).try_into().unwrap_or(std::u16::MAX);
            frame.dispose = gif::DisposalMethod::Background;

            frame
        })
        .collect::<Vec<_>>();

    for frame in new_frames {
        encoder
            .write_frame(&frame)
            .map_err(|e| FluxError::Other(e.to_string()))?;
    }

    encoder
        .write_raw_extension(
            AnyExtension(0xFE),
            &["Generated by Assyst Flux (jacher.io/assyst)".as_bytes()],
        )
        .map_err(|e| FluxError::Other(e.to_string()))?;

    drop(encoder);

    Ok(buf)
}
