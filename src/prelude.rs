pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type Context = (&'static [u8], usize);
pub use std::format as f;
