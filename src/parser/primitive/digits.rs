use crate::prelude::*;

use crate::parser::primitive::digit::digit;

use tailcall::tailcall;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <digits> ::=  <digit> <digits> | <digit> "_" <digits> | <digit>
/// ```
#[tailcall]
pub fn digits(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match digit(source, read_position) {
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
        Err(result) => Err(result),
    }
}

#[cfg(test)]
mod test_digits {
    use crate::parser::primitive::digits::digits;

    #[test]
    fn test_digits_parser() {
        assert!(digits(b"0", 0).is_ok());
        assert!(digits(b"1", 0).is_ok());
        assert!(digits(b"a", 0).is_err());
        assert!(digits(b"", 0).is_err());

        let case1 = digits(b"0123", 0).unwrap();

        assert_eq!(case1.1, 4);
    }
}
