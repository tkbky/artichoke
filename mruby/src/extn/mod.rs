use crate::interpreter::Mrb;
use crate::MrbError;

pub mod core;
pub mod stdlib;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    core::patch(interp)?;
    stdlib::patch(interp)?;
    Ok(())
}
