use crate::prelude::*;

use tailcall::tailcall;

use crate::parser::traits::Source;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <nonzero> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
/// ```
pub fn nonzero(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    if let Some(b'1'..=b'9') = source.get(index) {
        let index = index + 1;
        let parsed = source.slice(..index);
        let result = (parsed, index);

        Ok(result)
    } else {
        let result = Error::new(index);

        Err(result)
    }
}

#[cfg(test)]
mod test_nonzero {
    use crate::parser::number::nonzero;
    use crate::prelude::*;

    #[test]
    fn test_nonzero_parser() {
        assert!(nonzero(b"0", 0).is_err());

        assert!(nonzero(b"1", 0).is_ok());

        assert_eq!(nonzero(b"0", 0), Err(Error::new(0)));

        assert_eq!(nonzero(b"123", 0), Ok((b"1" as &[u8], 1)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <zero> ::= "0"
/// ```
pub fn zero(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    if let Some(b'0') = source.get(index) {
        let index = index + 1;
        let parsed = source.slice(..index);
        let result = (parsed, index);

        Ok(result)
    } else {
        let result = Error::new(index);

        Err(result)
    }
}

#[cfg(test)]
mod test_zero {
    use crate::parser::number::zero;
    use crate::prelude::*;

    #[test]
    fn test_zero_parser() {
        assert!(zero(b"0", 0).is_ok());

        assert!(zero(b"1", 0).is_err());

        assert_eq!(zero(b"0", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(zero(b"123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <digit> ::= <zero> | <nonzero>
/// ```
pub fn digit(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    zero(source, index)
        .or(nonzero(source, index))
        .map_err(|_| Error::new(index))
}

#[cfg(test)]
mod test_digit {
    use crate::parser::number::digit;
    use crate::prelude::*;

    #[test]
    fn test_digit_parser() {
        assert!(digit(b"0", 0).is_ok());

        assert!(digit(b"1", 0).is_ok());

        assert!(digit(b"a", 0).is_err());

        assert_eq!(digit(b"0", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(digit(b"123", 0), Ok((b"1" as &[u8], 1)));

        assert_eq!(digit(b"bqwe123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <digits> ::=  <digit> <digits> | <digit> "_" <digits> | <digit>
/// ```
#[tailcall]
pub fn digits(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    match digit(source, index) {
        Ok(parsed) => {
            let (_, index) = parsed;

            match digit(source, index) {
                Ok(_) => digits(source, index),

                Err(_) => {
                    if let Some(b'_') = source.get(index) {
                        let index = index + 1;

                        digits(source, index)
                    } else {
                        let parsed = source.slice(..index);
                        let result = (parsed, index);

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
    use crate::parser::number::digits;
    use crate::prelude::*;

    #[test]
    fn test_digits_parser() {
        assert!(digits(b"0", 0).is_ok());

        assert!(digits(b"1", 0).is_ok());

        assert!(digits(b"0123", 0).is_ok());

        assert!(digits(b"a", 0).is_err());

        assert!(digits(b"", 0).is_err());

        assert_eq!(digits(b"0", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(digits(b"0123", 0), Ok((b"0123" as &[u8], 4)));

        assert_eq!(digits(b"bqwe123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <natural> ::= <nonzero> <digits1> | <nonzero> "_" <digits1> | <digit>
/// ```
pub fn natural(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    match nonzero(source, index) {
        Ok(parsed) => {
            let (_, index) = parsed;

            match digit(source, index) {
                Ok(_) => digits(source, index),

                Err(_) => {
                    if let Some(b'_') = source.get(index) {
                        let index = index + 1;

                        digits(source, index)
                    } else {
                        let parsed = source.slice(..index);
                        let result = (parsed, index);

                        Ok(result)
                    }
                }
            }
        }
        Err(_) => digit(source, index),
    }
}

#[cfg(test)]
mod test_natural {
    use crate::parser::number::natural;
    use crate::prelude::*;

    #[test]
    fn test_natural_parser() {
        assert!(natural(b"0", 0).is_ok());

        assert!(natural(b"1", 0).is_ok());

        assert!(natural(b"0123", 0).is_ok());

        assert!(natural(b"a", 0).is_err());

        assert!(natural(b"", 0).is_err());

        assert_eq!(natural(b"0", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(natural(b"0123", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(natural(b"bqwe123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <integer> ::= "+" <natural> | "-" <natural> | <natural>
/// ```
pub fn integer(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    if let Some(&b) = source.get(index) {
        if b == b'+' || b == b'-' {
            let index = index + 1;
            natural(source, index)
        } else {
            natural(source, index)
        }
    } else {
        let result = Error::new(index);
        Err(result)
    }
}

#[cfg(test)]
mod test_integer {
    use crate::parser::number::integer;
    use crate::prelude::*;

    #[test]
    fn test_integer_parser() {
        assert!(integer(b"0", 0).is_ok());

        assert!(integer(b"1", 0).is_ok());

        assert!(integer(b"-1", 0).is_ok());

        assert!(integer(b"+1", 0).is_ok());

        assert!(integer(b"0123", 0).is_ok());

        assert!(integer(b"a", 0).is_err());

        assert!(integer(b"", 0).is_err());

        assert_eq!(integer(b"0", 0), Ok((b"0" as &[u8], 1)));

        assert_eq!(integer(b"-0123", 0), Ok((b"-0" as &[u8], 2)));

        assert_eq!(integer(b"bqwe123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <float> ::= <integer> "." <digits1>
/// ```
pub fn float(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    match integer(source, index) {
        Ok(parsed) => {
            let (_, index) = parsed;

            if let Some(b'.') = source.get(index) {
                let index = index + 1;

                digits(source, index)
            } else {
                let result = Error::new(index);

                Err(result)
            }
        }
        Err(result) => Err(result),
    }
}

#[cfg(test)]
mod test_float {
    use crate::parser::number::float;
    use crate::prelude::*;

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

        assert_eq!(float(b"0.1", 0), Ok((b"0.1" as &[u8], 3)));

        assert_eq!(float(b"-0123", 0), Err(Error::new(2)));

        assert_eq!(float(b"-123.1", 0), Ok((b"-123.1" as &[u8], 6)));

        assert_eq!(float(b"bqwe123", 0), Err(Error::new(0)));
    }
}

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <number> ::= <float> | <integer>
/// ```
pub fn number(source: &'_ [u8], index: usize) -> Result<Parsed<&'_ [u8]>> {
    float(source, index)
        .or(integer(source, index))
        .map_err(|_| Error::new(index))
}
