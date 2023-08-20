use crate::prelude::*;

use crate::parser::primitive::{nonzero, zero};

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <digit> ::= <zero> | <nonzero>
/// ```
pub fn digit(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    zero(source, read_position)
        .or(nonzero(source, read_position))
        .map_err(|_| match source.get(read_position) {
            Some(byte) => Error::Generic(f!("expected a digit, got '{}'", *byte as char)),
            None => Error::Generic("expected a digit, got none".to_string()),
        })
}

#[cfg(test)]
mod test_digit {
    use crate::parser::primitive::digit::digit;

    #[test]
    fn test_digit_parser() {
        assert!(digit(b"0", 0).is_ok());
        assert!(digit(b"1", 0).is_ok());
        assert!(digit(b"a", 0).is_err());
        assert!(digit(b"", 0).is_err());
    }
}
