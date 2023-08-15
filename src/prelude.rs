pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type SourceRef<'a> = &'a [u8];

pub type Index = usize;

pub use std::format as f;
