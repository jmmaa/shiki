use crate::prelude::*;

use crate::parser::primitive::{digit::digit, digits::digits, nonzero::nonzero};

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <natural> ::= <nonzero> <digits1> | <nonzero> "_" <digits1> | <digit>
/// ```
pub fn natural(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match nonzero(source, read_position) {
        Ok(result) => {
            let (source, read_position) = result;

            match digit(source, read_position) {
                Ok(_) => digits(source, read_position),

                Err(_) => {
                    if let Some(b'_') = source.get(read_position) {
                        digits(source, read_position + 1)
                    } else {
                        let result = (source, read_position);

                        Ok(result)
                    }
                }
            }
        }
        Err(_) => digit(source, read_position),
    }
}

#[cfg(test)]
mod test_natural {
    use crate::parser::primitive::natural::natural;

    #[test]
    fn test_natural_parser() {
        assert!(natural(b"0", 0).is_ok());
        assert!(natural(b"1", 0).is_ok());
        assert!(natural(b"0123", 0).is_ok());
        assert!(natural(b"a", 0).is_err());
        assert!(natural(b"", 0).is_err());
        assert!(natural(b"bqwe123", 0).is_err());

        let case1 = natural(b"0", 0).unwrap();

        assert_eq!(case1.1, 1);

        let case2 = natural(b"0123", 0).unwrap();

        assert_eq!(case2.1, 1);

        let case3 = natural(b"1234", 0).unwrap();

        assert_eq!(case3.1, 4);
    }
}
