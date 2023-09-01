//!
//! This project is inspired from the Computerphile Video of Graham Hutton
//! Link: https://youtu.be/dDtZLm7HIJs?si=khUftQbWYf-87NHZ
//!
//!
//! Basically, this is a markup language made from multiple little parsers. To improve from the idea,
//! I used slices instead of strings, this is to minimize memory allocation overhead.
//!
//! So instead of this signature based from the video:
//! ```rust
//! type Parser<T> = fn(String) -> (T, String);
//! ```
//!
//! The parsers will follow this one:
//!
//! ```rust
//! type Parser<T> = fn(Source, Position) -> (T, Source, Position);
//!     
//! type Source<'src> = &'src [u8]; // The reference on source bytes
//!
//! type Position = usize; // The current reading position
//! ```
//! Some parsers will have different signature depending on its needs, but it will still implement
//! a superset of type `Parser<T>`.
pub mod error;
pub mod parser;
pub mod prelude;
pub mod utils;
