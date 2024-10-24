use validator::Validate;

use crate::error::Result;

pub mod encryption;
pub mod jwt;

pub fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}