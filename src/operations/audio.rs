use crate::core::media_container::MediaContainer;
use crate::processing::ffmpeg::{drip, femurbreaker, siren, sweden, terraria};
use crate::processing::media_object::MediaObject;

use super::OperationResult;

impl MediaContainer {
    pub fn drip(&self) -> OperationResult {
        let input = self.pop_input()?;
        let input = input.encode(&self.limits)?;

        let out = drip(&input)?;

        Ok(MediaObject::Encoded(out))
    }

    pub fn femurbreaker(&self) -> OperationResult {
        let input = self.pop_input()?;
        let input = input.encode(&self.limits)?;

        let out = femurbreaker(&input)?;

        Ok(MediaObject::Encoded(out))
    }

    pub fn siren(&self) -> OperationResult {
        let input = self.pop_input()?;
        let input = input.encode(&self.limits)?;

        let out = siren(&input)?;

        Ok(MediaObject::Encoded(out))
    }

    pub fn sweden(&self) -> OperationResult {
        let input = self.pop_input()?;
        let input = input.encode(&self.limits)?;

        let out = sweden(&input)?;

        Ok(MediaObject::Encoded(out))
    }

    pub fn terraria(&self) -> OperationResult {
        let input = self.pop_input()?;
        let input = input.encode(&self.limits)?;

        let out = terraria(&input)?;

        Ok(MediaObject::Encoded(out))
    }
}
