// finish letter

use crate::prelude::*;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <letter> ::= <lowercase_letter> | <uppercase_letter>
/// ```
pub fn letter(source: &'static [u8], read_position: usize) -> Result<Context> {
    match source.get(read_position) {
        Some(byte) => {
            if byte.is_ascii_alphabetic() {
                let result = (source, read_position + 1);

                Ok(result)
            } else {
                let result = Error::Generic(f!("expected a letter, got '{}'", *byte as char));

                Err(result)
            }
        }
        None => {
            let result = Error::Generic("expected a letter, got none".to_string());

            Err(result)
        }
    }
}

#[cfg(test)]
mod test_letter {

    use crate::parser::primitive::letter;

    #[test]
    fn test_letter_parser() {
        assert!(letter(b"a", 0).is_ok());
        assert!(letter(b"A", 0).is_ok());
        assert!(letter(b"&", 0).is_err());
        assert!(letter(b"0", 0).is_err());
        assert!(letter(b"1", 0).is_err());
        assert!(letter(b"", 0).is_err());
    }
}