use std::collections::HashMap;

use anyhow::Context;
use bloom::BloomOptions;
use resize::ResizeOptions;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

pub mod ah_shit;
pub mod april_fools;
pub mod audio;
pub mod bloom;
pub mod blur;
pub mod caption;
pub mod deepfry;
pub mod fisheye;
pub mod flip_flop;
pub mod frame_shift;
pub mod frames;
pub mod ghost;
pub mod makesweet;
pub mod meme;
pub mod resize;
pub mod reverse;

pub type OperationResult = Result<MediaObject, FluxError>;

fn option_get_u64(options: &HashMap<String, String>, name: &str) -> anyhow::Result<Option<u64>> {
    options
        .get(name)
        .map(|x| {
            x.parse::<u64>()
                .with_context(|| format!("Failed to parse {name} (invalid u64 {})", options.get(name).unwrap()))
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
            "back-tattoo" => self.back_tattoo()?,
            "billboard" => self.billboard()?,
            "bloom" => {
                let radius = option_get_u64(&options, "radius")?;
                let brightness = option_get_u64(&options, "brightness")?;
                let sharpness = option_get_u64(&options, "sharpness")?;

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
            "book" => self.book()?,
            "caption" => {
                let text = options.get("text").ok_or(FluxError::ParameterError(
                    "Missing required option text for operation caption".to_owned(),
                ))?;

                self.caption(&text[..])?
            },
            "circuitboard" => self.circuitboard()?,
            "deepfry" => self.deepfry()?,
            "drip" => self.drip()?,
            "femurbreaker" => self.femurbreaker()?,
            "fisheye" => self.fisheye()?,
            "flag" => self.flag()?,
            "flag2" => self.flag2()?,
            "flip" => self.flip()?,
            "flop" => self.flop()?,
            "fortune-cookie" => self.fortune_cookie()?,
            "frame-shift" => self.frame_shift()?,
            "frames" => self.frames()?,
            "ghost" => {
                let depth = option_get_u64(&options, "depth")?;

                self.ghost(depth)?
            },
            "heart-locket" => {
                let text = options.get("text").map(|x| x.clone());

                self.heart_locket(text)?
            },
            "meme" => {
                let top = options.get("top").map(|x| x.clone());
                let bottom = options.get("bottom").map(|x| x.clone());

                self.meme(top, bottom)?
            },
            "resize" => {
                let width = option_get_u64(&options, "width")?;
                let height = option_get_u64(&options, "height")?;
                let scale = option_get_f32(&options, "scale")?;

                let resize_options = ResizeOptions { width, height, scale };

                self.resize(resize_options)?
            },
            "reverse" => self.reverse()?,
            "rubiks" => self.rubiks()?,
            "siren" => self.siren()?,
            "sweden" => self.siren()?,
            "terraria" => self.terraria()?,
            "toaster" => self.toaster()?,
            "valentine" => self.valentine()?,
            _ => Err(FluxError::ParameterError(format!("Unrecognised operation {operation}")))?,
        })
    }
}
