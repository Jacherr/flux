use std::collections::HashMap;

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
pub mod general;
pub mod ghost;
pub mod gif;
pub mod globe;
pub mod grayscale;
pub mod info;
pub mod invert;
pub mod jpeg;
pub mod magik;
pub mod makesweet;
pub mod meme;
pub mod motivate;
pub mod neon;
pub mod overlay;
pub mod paint;
pub mod ping_pong;
pub mod pixelate;
pub mod posterize;
pub mod rainbow;
pub mod resize;
pub mod reverse;
pub mod rotate;
pub mod scramble;
pub mod set_loop;
pub mod speech_bubble;
pub mod speed;
pub mod spin;
pub mod spread;
pub mod swirl;
pub mod uncaption;
pub mod wormhole;
pub mod zoom;
pub mod zoom_blur;

pub type OperationResult = Result<MediaObject, FluxError>;

fn option_get_bool(options: &HashMap<String, String>, name: &str) -> Result<bool, FluxError> {
    let op = options.get(name);

    if let Some(op) = op {
        if op == "1" {
            Ok(true)
        } else if op == "0" {
            Ok(false)
        } else {
            Err(FluxError::ParameterError(format!(
                "Invalid value {op} for option {name}: expected either 0 or 1 "
            )))
        }
    } else {
        Ok(false)
    }
}

fn option_get_u64(options: &HashMap<String, String>, name: &str) -> Result<Option<u64>, FluxError> {
    options
        .get(name)
        .map(|x| {
            x.parse::<u64>().map_err(|_| {
                FluxError::ParameterError(format!(
                    "Failed to parse {name} (invalid uint64 {})",
                    options.get(name).unwrap()
                ))
            })
        })
        .transpose()
}

fn option_get_f32(options: &HashMap<String, String>, name: &str) -> Result<Option<f32>, FluxError> {
    options
        .get(name)
        .map(|x| {
            x.parse::<f32>().map_err(|_| {
                FluxError::ParameterError(format!(
                    "Failed to parse {name} (invalid float32 {})",
                    options.get(name).unwrap()
                ))
            })
        })
        .transpose()
}

fn option_get_i64(options: &HashMap<String, String>, name: &str) -> Result<Option<i64>, FluxError> {
    options
        .get(name)
        .map(|x| {
            x.parse::<i64>().map_err(|_| {
                FluxError::ParameterError(format!(
                    "Failed to parse {name} (invalid int64 {})",
                    options.get(name).unwrap()
                ))
            })
        })
        .transpose()
}

fn option_get_str<'a>(options: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    options.get(name).map(|x| x.as_str())
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
                let bottom = option_get_bool(&options, "bottom")?;
                let black = option_get_bool(&options, "black")?;

                self.caption(&text[..], bottom, black)?
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
            "gif" => self.gif()?,
            "gif-magik" => self.gif_magik()?,
            "globe" => self.globe()?,
            "grayscale" => self.grayscale()?,
            "heart-locket" => {
                let text = options.get("text").map(|x| x.clone());

                self.heart_locket(text)?
            },
            "invert" => self.invert()?,
            "jpeg" => {
                let quality = option_get_u64(&options, "quality")?;

                self.jpeg(quality)?
            },
            "magik" => self.magik()?,
            "meme" => {
                let top = options.get("top").map(|x| x.clone());
                let bottom = options.get("bottom").map(|x| x.clone());

                self.meme(top, bottom)?
            },
            "motivate" => {
                let top = options.get("top").map(|x| x.clone());
                let bottom = options.get("bottom").map(|x| x.clone());

                self.motivate(top, bottom)?
            },
            "neon" => self.neon()?,
            "overlay" => self.overlay()?,
            "paint" => self.paint()?,
            "ping-pong" => self.ping_pong()?,
            "pixelate" => {
                let strength = option_get_f32(&options, "strength")?;

                self.pixelate(strength)?
            },
            "posterize" => self.posterize(option_get_str(&options, "cols").unwrap())?,
            "rainbow" => self.rainbow()?,
            "resize" => {
                let width = option_get_u64(&options, "width")?;
                let height = option_get_u64(&options, "height")?;
                let scale = option_get_f32(&options, "scale")?;

                let resize_options = ResizeOptions { width, height, scale };

                self.resize(resize_options)?
            },
            "reverse" => self.reverse()?,
            "rotate" => {
                let deg = option_get_u64(&options, "degrees")?;

                self.rotate(deg)?
            },
            "rubiks" => self.rubiks()?,
            "scramble" => self.scramble()?,
            "set-loop" => {
                let loops = option_get_i64(&options, "loops")?;

                self.set_loop(loops.unwrap_or(-1))?
            },
            "siren" => self.siren()?,
            "speech-bubble" => {
                let t = option_get_bool(&options, "solid")?;
                self.speech_bubble(t)?
            },
            "speed" => {
                let multiplier = option_get_f32(&options, "multiplier")?;

                self.speed(multiplier.map(|m| m.into()))?
            },
            "spin" => self.spin()?,
            "spread" => {
                let strength = option_get_u64(&options, "strength")?;

                self.spread(strength)?
            },
            "sweden" => self.sweden()?,
            "swirl" => {
                let strength = option_get_f32(&options, "strength")?;

                self.swirl(strength)?
            },
            "terraria" => self.terraria()?,
            "toaster" => self.toaster()?,
            "uncaption" => {
                let amount = option_get_str(&options, "amount");

                self.uncaption(amount)?
            },
            "valentine" => self.valentine()?,
            "wormhole" => self.wormhole()?,
            "zoom" => self.zoom()?,
            "zoom-blur" => {
                let power = option_get_f32(&options, "power")?;

                self.zoom_blur(power)?
            },
            // general ops
            "threshold" => self.threshold(option_get_f32(&options, "threshold")?)?,
            "channels" => self.channels(option_get_str(&options, "keep"))?,
            "edges" => self.edges()?,
            _ => Err(FluxError::ParameterError(format!("Unrecognised operation {operation}")))?,
        })
    }
}
