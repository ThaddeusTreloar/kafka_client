use std::fs::File;

use crate::{error::Error, prelude::Result};

// TODO : For the raw kafka config format .ini?
pub struct RawConfig {}

impl TryFrom<String> for RawConfig {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        unimplemented!()
    }
}

impl TryFrom<File> for RawConfig {
    type Error = Error;

    fn try_from(value: File) -> Result<Self> {
        unimplemented!()
    }
}
