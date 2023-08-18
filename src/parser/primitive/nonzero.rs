use crate::prelude::*;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <nonzero> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
/// ```
pub fn nonzero(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match source.get(read_position) {
        Some(byte) => match byte {
            b'1'..=b'9' => {
                let result = (source, read_position + 1);

                Ok(result)
            }
            _ => {
                let result = Error::Generic(f!("expected a nonzero, got '{}'", *byte as char));

                Err(result)
            }
        },

        None => {
            let result = Error::Generic("expected a nonzero, got none".to_string());

            Err(result)
        }
    }
}

#[cfg(test)]
mod test_nonzero {
    use crate::parser::primitive::nonzero::nonzero;

    #[test]
    fn test_nonzero_parser() {
        assert!(nonzero(b"0", 0).is_err());
        assert!(nonzero(b"1", 0).is_ok());

        let case1 = nonzero(b"1", 0).unwrap();

        assert_eq!(case1.1, 1);
    }
}
