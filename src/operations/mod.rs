use std::collections::HashMap;

use anyhow::Context;

use crate::core::error::FluxError;
use crate::core::media_container::MediaContainer;
use crate::processing::media_object::MediaObject;

pub mod ah_shit;
pub mod blur;
pub mod reverse;

pub type OperationResult = Result<MediaObject, FluxError>;

impl MediaContainer {
    pub fn perform_operation(
        &self,
        operation: &str,
        options: HashMap<String, String>,
    ) -> Result<MediaObject, FluxError> {
        Ok(match operation {
            "ah-shit" => self.ah_shit()?,
            "blur" => {
                let strength = options
                    .get("strength")
                    .map(|x| {
                        x.parse::<f32>().with_context(|| {
                            format!(
                                "Failed to parse blur strength (invalid float {})",
                                options.get("strength").unwrap()
                            )
                        })
                    })
                    .transpose()?;

                self.blur(strength)?
            },
            "reverse" => self.reverse()?,
            _ => unimplemented!(),
        })
    }
}
