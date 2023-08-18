use crate::prelude::*;

use crate::parser::primitive::natural::natural;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <integer> ::= "+" <natural> | "-" <natural> | <natural>
/// ```
pub fn integer(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    if let Some(&b) = source.get(read_position) {
        if b == b'+' || b == b'-' {
            let read_position = read_position + 1;
            natural(source, read_position)
        } else {
            natural(source, read_position)
        }
    } else {
        let result = Error::Generic("expected an integer, got none".to_string());

        Err(result)
    }
}

#[cfg(test)]
mod test_integer {
    use crate::parser::primitive::integer::integer;

    #[test]
    fn test_integer_parser() {
        assert!(integer(b"0", 0).is_ok());
        assert!(integer(b"1", 0).is_ok());
        assert!(integer(b"-1", 0).is_ok());
        assert!(integer(b"+1", 0).is_ok());
        assert!(integer(b"0123", 0).is_ok());
        assert!(integer(b"a", 0).is_err());
        assert!(integer(b"", 0).is_err());
        assert!(integer(b"-", 0).is_err());
        assert!(integer(b"+", 0).is_err());

        let case1 = integer(b"0", 0).unwrap();

        assert_eq!(case1.1, 1);

        let case2 = integer(b"-0123", 0).unwrap();

        assert_eq!(case2.1, 2);

        let case3 = integer(b"+123", 0).unwrap();

        assert_eq!(case3.1, 4);

        let case4 = integer(b"-123.", 0).unwrap(); // TAKE NOTE

        assert_eq!(case4.1, 4);
    }
}
