use std::time::Duration;

use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::ffmpeg_operations;
use crate::processing::media_object::MediaObject;
use crate::util::{keep_every_nth_in_vec, remove_every_nth_from_vec};

use super::OperationResult;

impl MediaContainer {
    pub fn speed(&self, multiplier: Option<f64>) -> OperationResult {
        let input = self.pop_input()?;
        let multiplier = multiplier.unwrap_or(1.5).clamp(0.1, 15.0);

        if let Some(v) = input.try_encoded_video(self.limits.video_decode_permitted) {
            return Ok(MediaObject::Encoded(ffmpeg_operations::speed_video(v?, multiplier)?));
        };

        let mut dyn_images = input.to_dynamic_images(&self.limits)?.into_owned();
        let len = dyn_images.images.len();

        let rm_frames;
        if dyn_images.maybe_first()?.1.unwrap_or_default().as_millis() == 20 && multiplier > 1.0 {
            rm_frames = true;
        } else {
            rm_frames = false;
        }

        for image in dyn_images.images.iter_mut().enumerate() {
            image.1.1 = Some(Duration::from_millis(
                (image.1.1.unwrap_or_default().as_millis() as f64 * (1.0 / multiplier))
                    .floor()
                    .clamp(20.0, 100000.0) as u64,
            ));
        }

        let mut n = -1;
        let mut s = -1;
        if multiplier > 1.0 && rm_frames {
            let frames_left = (len as f64 * (1.0 / multiplier)).ceil() as i32;
            let mut overshoot_k = (-1, -1);
            let mut undershoot_k = (-1, -1);
            let mut overshoot_r = (-1, -1);
            let mut undershoot_r = (-1, -1);
            for i in (1..(len as i32)).rev() {
                let mut v = vec![0; len];
                v = keep_every_nth_in_vec(&v, i as usize);
                if v.len() as i32 > frames_left {
                    overshoot_k = (i, v.len() as i32);
                    break;
                } else if (v.len() as i32) < frames_left {
                    undershoot_k = (i, v.len() as i32);
                }
            }
            for i in 1..(len as i32) {
                let mut v = vec![0; len];
                remove_every_nth_from_vec(&mut v, i as usize);
                if v.len() as i32 > frames_left {
                    overshoot_r = (i, v.len() as i32);
                    break;
                } else if (v.len() as i32) < frames_left {
                    undershoot_r = (i, v.len() as i32);
                }
            }
            let ko_dif = (overshoot_k.1 - frames_left).abs();
            let ku_dif = (undershoot_k.1 - frames_left).abs();
            let kn;
            let kw;
            if ko_dif >= ku_dif {
                kw = ku_dif;
                kn = undershoot_k.0;
            } else {
                kw = ko_dif;
                kn = overshoot_k.0;
            };
            let ro_dif = (overshoot_r.1 - frames_left).abs();
            let ru_dif = (undershoot_r.1 - frames_left).abs();
            let rn;
            let rw;
            if ro_dif >= ru_dif {
                rw = ru_dif;
                rn = undershoot_r.0;
            } else {
                rw = ro_dif;
                rn = overshoot_r.0;
            };
            if rw > kw {
                n = kn;
                s = 1;
            } else {
                n = rn;
                s = 0;
            };
        }

        if n != -1 {
            if s == 1 {
                let n = keep_every_nth_in_vec(&dyn_images.images, n as usize);
                dyn_images.images = n;
            } else {
                remove_every_nth_from_vec(&mut dyn_images.images, n as usize);
            };
        };

        Ok(MediaObject::DynamicImages(dyn_images))
    }
}
