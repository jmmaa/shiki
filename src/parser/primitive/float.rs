use crate::prelude::*;

use crate::parser::primitive::{digits::digits, integer::integer};

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <float> ::= <integer> "." <digits1>
/// ```
pub fn float(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match integer(source, read_position) {
        Ok(result) => {
            let (source, read_position) = result;

            if let Some(&b) = source.get(read_position) {
                if b == b'.' {
                    digits(source, read_position + 1)
                } else {
                    let result = Error::Generic(f!("expected a '.', got '{}'", b as char));

                    Err(result)
                }
            } else {
                let result = Error::Generic("expected a '.', got none".to_string());

                Err(result)
            }
        }
        Err(result) => Err(result),
    }
}

#[cfg(test)]
mod test_float {
    use crate::parser::primitive::float::float;

    #[test]
    fn test_float_parser() {
        assert!(float(b"0", 0).is_err());
        assert!(float(b"1", 0).is_err());
        assert!(float(b"-1", 0).is_err());
        assert!(float(b"0.1", 0).is_ok());
        assert!(float(b"1.0", 0).is_ok());
        assert!(float(b"-1.", 0).is_err());
        assert!(float(b"+1", 0).is_err());
        assert!(float(b"0123", 0).is_err());
        assert!(float(b"a", 0).is_err());
        assert!(float(b"", 0).is_err());
        assert!(float(b"-", 0).is_err());
        assert!(float(b"+", 0).is_err());
        assert!(float(b"-1.13", 0).is_ok());
        assert!(float(b"+1123.443", 0).is_ok());

        let case1 = float(b"0.1", 0).unwrap();

        assert_eq!(case1.1, 3);

        let case2 = float(b"-0.123", 0).unwrap();

        assert_eq!(case2.1, 6);

        let case3 = float(b"+123.1", 0).unwrap();

        assert_eq!(case3.1, 6);
    }
}
