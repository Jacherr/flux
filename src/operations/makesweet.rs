use std::io::Cursor;

use image::ImageFormat;

use crate::core::media_container::MediaContainer;
use crate::processing::makesweet::{
    back_tattoo, billboard_cityscape, book, circuitboard, flag, flag2, fortune_cookie, heart_locket, rubiks, toaster,
    valentine,
};
use crate::processing::media_object::MediaObject;
use crate::vips::vips_generate_heart_locket_text;

use super::OperationResult;

impl MediaContainer {
    pub fn back_tattoo(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = back_tattoo(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn billboard(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = billboard_cityscape(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn book(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = book(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn circuitboard(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = circuitboard(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn flag(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = flag(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn flag2(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = flag2(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn fortune_cookie(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = fortune_cookie(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn heart_locket(&self, text: Option<String>) -> OperationResult {
        let input1 = self
            .pop_input()?
            .encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;

        let input2 = if let Some(text) = text {
            let mut out = Vec::new();
            vips_generate_heart_locket_text(&text, 250, 250)?
                .to_rgb8()
                .write_to(&mut Cursor::new(&mut out), ImageFormat::Jpeg)?;

            out
        } else {
            self.pop_input()?
                .encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?
        };

        let result = heart_locket(&input1, &input2)?;
        Ok(MediaObject::Encoded(result))
    }

    pub fn rubiks(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = rubiks(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn toaster(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = toaster(&enc)?;

        Ok(MediaObject::Encoded(result))
    }

    pub fn valentine(&self) -> OperationResult {
        let input = self.pop_input()?;

        let enc = input.encode_first_frame_as(ImageFormat::Jpeg, &self.limits)?;
        let result = valentine(&enc)?;

        Ok(MediaObject::Encoded(result))
    }
}
