use std::collections::HashMap;

use anyhow::Context;
use bloom::BloomOptions;
use resize::ResizeOptions;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

pub mod ah_shit;
pub mod april_fools;
pub mod bloom;
pub mod blur;
pub mod caption;
pub mod resize;
pub mod reverse;

pub type OperationResult = Result<MediaObject, FluxError>;

fn option_get_usize(options: &HashMap<String, String>, name: &str) -> anyhow::Result<Option<usize>> {
    options
        .get(name)
        .map(|x| {
            x.parse::<usize>()
                .with_context(|| format!("Failed to parse {name} (invalid usize {})", options.get(name).unwrap()))
        })
        .transpose()
}

fn option_get_f32(options: &HashMap<String, String>, name: &str) -> anyhow::Result<Option<f32>> {
    options
        .get(name)
        .map(|x| {
            x.parse::<f32>().with_context(|| {
                format!(
                    "Failed to parse {name} (invalid float32 {})",
                    options.get(name).unwrap()
                )
            })
        })
        .transpose()
}

impl MediaContainer {
    pub fn perform_operation(
        &self,
        operation: &str,
        options: HashMap<String, String>,
    ) -> Result<MediaObject, FluxError> {
        Ok(match operation {
            "ah-shit" => self.ah_shit()?,
            "april-fools" => self.april_fools()?,
            "bloom" => {
                let radius = option_get_usize(&options, "radius")?;
                let brightness = option_get_usize(&options, "brightness")?;
                let sharpness = option_get_usize(&options, "sharpness")?;

                let bloom_options = BloomOptions {
                    radius,
                    brightness,
                    sharpness,
                };

                self.bloom(bloom_options)?
            },
            "blur" => {
                let strength = option_get_f32(&options, "strength")?;

                self.blur(strength)?
            },
            "caption" => {
                let text = options.get("text").ok_or(FluxError::ParameterError(
                    "Missing required option text for operation caption".to_owned(),
                ))?;

                self.caption(&text[..])?
            },
            "resize" => {
                let width = option_get_usize(&options, "width")?;
                let height = option_get_usize(&options, "height")?;
                let scale = option_get_f32(&options, "scale")?;

                let resize_options = ResizeOptions { width, height, scale };

                self.resize(resize_options)?
            },
            "reverse" => self.reverse()?,
            _ => unimplemented!(),
        })
    }
}
