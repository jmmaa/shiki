use crate::prelude::*;

use crate::parser::primitive::{digit, letter};

///```bnf
/// <alphanumeric> ::= <digit> | <letter>
///```
pub fn alphanumeric(source: &'static [u8], read_position: usize) -> Result<Context> {
    letter(source, read_position)
        .or(digit(source, read_position))
        .map_err(|_| match source.get(read_position) {
            Some(byte) => Error::Generic(f!("expected a letter or digit, got '{}'", *byte as char)),
            None => Error::Generic("expected a letter or digit, got none".to_string()),
        })
}
