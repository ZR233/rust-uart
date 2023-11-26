use kernel::error::*;
use kernel::prelude::*;
use kernel::{bindings::*, init::PinInit};

pub struct Port {}

impl Port {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    pub fn register(&self)->Result{
        Ok(())
    }
}
