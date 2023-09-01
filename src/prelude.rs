pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type Parser<T> = fn(Source, Position) -> Result<(T, Source, Position)>;

pub type Source<'src> = &'src [u8];

pub type Position = usize;

pub use std::format as f;
