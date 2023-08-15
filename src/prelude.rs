pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type Parsed<T> = (T, usize);
