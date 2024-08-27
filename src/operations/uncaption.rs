use image::{load_from_memory, DynamicImage};
use imageproc::edges::canny;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::{self, ffmpeg_operations};
use crate::processing::media_object::MediaObject;

use super::OperationResult;

fn uncaption_get_row(image: &DynamicImage) -> Result<u32, FluxError> {
    let canny = canny(&image.to_luma8(), 0.5, 1.5);
    let mut row_num: u32 = 0;

    for (idx, row) in canny.rows().enumerate() {
        // first 5 pixels
        for px in row.take(5) {
            let lightness = px.0;
            if lightness[0] > 0 {
                row_num = idx as u32 + 1;
                return Ok(row_num);
            }
        }
    }

    Ok(row_num)
}

fn uncaption_video(v: &[u8], amount: Option<&str>) -> Result<Vec<u8>, FluxError> {
    let frame1 = ffmpeg::get_video_first_frame(v)?;
    let image = load_from_memory(&frame1)?;
    let row = if let Some(a) = amount {
        parse_amount_to_lines(image.height(), a)?
    } else {
        uncaption_get_row(&image)?
    };
    ffmpeg_operations::crop_video(v, 0, row as _, image.width() as _, (image.height() - row) as _)
}

fn parse_amount_to_lines(height: u32, amount: &str) -> Result<u32, FluxError> {
    let row_num = if amount.ends_with("%") {
        let n2 = amount
            .trim_end_matches("%")
            .parse::<f32>()
            .map_err(|_| FluxError::ParameterError("Invalid input: Should be number of lines or a %".to_owned()))?;
        let multiplier = n2 / 100.0;
        ((height as f32) * multiplier) as u32
    } else {
        let n2 = amount
            .parse::<usize>()
            .map_err(|_| FluxError::ParameterError("Invalid input: Should be number of lines or a %".to_owned()))?;
        n2.clamp(0, height as usize) as u32
    };

    if row_num == 0 {
        return Err(FluxError::InputMediaError(
            r#"No caption boundary found. The image must have a clear boundary between the image and the caption.
        Alternatively you can provide an amount of lines to use in your command - look at uncaption help for details"#
                .to_string(),
        ));
    } else if row_num >= height {
        return Err(FluxError::ParameterError(
            r#"Cannot remove more rows than are present in the image."#.to_string(),
        ));
    };

    Ok(row_num)
}

impl MediaContainer {
    pub fn uncaption(&self, amount: Option<&str>) -> OperationResult {
        let input = self.pop_input()?;

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return uncaption_video(v?, amount).map(MediaObject::Encoded);
        }

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let first = &dyn_images.maybe_first()?.0;
        let height = first.height();

        // bounds checking happens within these functions
        let row = if let Some(a) = amount {
            parse_amount_to_lines(height, a)?
        } else {
            uncaption_get_row(&first)?
        };

        if row >= height {
            return Err(FluxError::InputMediaError("No caption boundary found.".to_owned()));
        }

        dyn_images.iter_images_mut(|f, _| f.crop(0, row, f.width(), f.height()));

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
