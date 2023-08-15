use crate::prelude::*;
use tailcall::tailcall;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <nonzero> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
/// ```
pub fn nonzero<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    match s.get(i) {
        Some(b'1'..=b'9') => {
            let i = i + 1;

            match s.get(..i) {
                Some(v) => Ok((v, i)),
                None => Err(Error::new(i)),
            }
        }
        _ => Err(Error::new(i)),
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
pub fn zero<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    match s.get(i) {
        Some(b'0') => {
            let i = i + 1;

            match s.get(..i) {
                Some(v) => Ok((v, i)),
                None => Err(Error::new(i)),
            }
        }
        _ => Err(Error::new(i)),
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
pub fn digit<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    zero(s, i).or(nonzero(s, i)).or_else(|_| Err(Error::new(i)))
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
pub fn digits<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    match digit(s, i) {
        Ok((_, i)) => match digit(s, i) {
            Ok(_) => digits(s, i),
            Err(_) => {
                if let Some(&b) = s.get(i) {
                    if b == b'_' {
                        let i = i + 1;

                        digits(s, i)
                    } else {
                        Ok((&s[..i], i))
                    }
                } else {
                    Ok((&s[..i], i))
                }
            }
        },
        Err(e) => Err(e),
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
pub fn natural<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    match nonzero(s, i) {
        Ok((_, i)) => match digit(s, i) {
            Ok(_) => digits(s, i),
            Err(_) => {
                if let Some(&b) = s.get(i) {
                    if b == b'_' {
                        let i = i + 1;

                        digits(s, i)
                    } else {
                        Ok((&s[..i], i))
                    }
                } else {
                    Ok((&s[..i], i))
                }
            }
        },
        Err(_) => digit(s, i),
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
pub fn integer<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    if let Some(&b) = s.get(i) {
        if b == b'-' || b == b'+' {
            let i = i + 1;
            natural(s, i)
        } else {
            natural(s, i)
        }
    } else {
        Err(Error::new(i))
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
pub fn float<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    match integer(s, i) {
        Ok((_, i)) => {
            if let Some(&b) = s.get(i) {
                if b == b'.' {
                    let i = i + 1;
                    digits(s, i)
                } else {
                    Err(Error::new(i))
                }
            } else {
                Err(Error::new(i))
            }
        }
        Err(e) => Err(e),
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
pub fn number<'a>(s: SourceRef<'a>, i: Index) -> Result<(&'a [u8], Index)> {
    float(s, i)
        .or(integer(s, i))
        .or_else(|_| Err(Error::new(i)))
}

// add tests for number
