use crate::prelude::*;

use crate::utils::ByteUtil;

use tailcall::tailcall;

/*


<number> ::= <integer> <fraction> <exponent>

<fraction> ::= E | "." <digits>

<exponent> ::= E | "e" <sign> <digits> | "E" <sign> <digits>

<integer> ::= <sign> <natural>

<sign> ::= "-" | "+" | E

<natural> ::= <nonzero> <digits> | <nonzero> "_" <digits> | <digit>

<digits> ::=  <digit> <digits> | <digit> "_" <digits> | <digit>



*/

/// <digits> ::=  <digit> <digits> | <digit> "_" <digits> | <digit>
#[tailcall]
fn digits(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is_ascii_digit() {
            let pos = pos + 1;

            if let Some(byte) = src.get(pos) {
                if byte.is_ascii_digit() {
                    digits(src, pos)
                } else if byte.is('_') {
                    digits(src, pos + 1)
                } else {
                    let parsed = &src[..pos];
                    let result = (parsed, src, pos);

                    Ok(result)
                }
            } else {
                let parsed = &src[..pos];
                let result = (parsed, src, pos);

                Ok(result)
            }
        } else {
            let result = Error::Generic(f!("expected a digit, got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a digit, got none".to_string());

        Err(result)
    }
}

#[test]
fn test_digits() {
    assert!(digits(b"0", 0).is_ok());
    assert!(digits(b"1", 0).is_ok());

    assert!(digits(b"0123", 0).is_ok());
    assert!(digits(b"123", 0).is_ok());

    assert!(digits(b"_0123", 0).is_err());
    assert!(digits(b"123_", 0).is_err());

    assert!(digits(b"", 0).is_err());

    let case = digits(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = digits(b"0123", 0).unwrap();
    assert_eq!(case.0, b"0123");

    let case = digits(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = digits(b"123", 0).unwrap();
    assert_eq!(case.0, b"123");

    let case = digits(b"1_23", 0).unwrap();
    assert_eq!(case.0, b"1_23");

    let case = digits(b"123_123", 0).unwrap();
    assert_eq!(case.0, b"123_123");
}

/// <natural> ::= <nonzero> <digits> | <nonzero> "_" <digits> | <digit>
#[inline(always)]
pub fn natural(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is_ascii_digit() && byte.is_not('0') {
            let pos = pos + 1;

            if let Some(byte) = src.get(pos) {
                if byte.is_ascii_digit() {
                    digits(src, pos)
                } else if byte.is('_') {
                    digits(src, pos + 1)
                } else {
                    let parsed = &src[..pos];
                    let result = (parsed, src, pos);

                    Ok(result)
                }
            } else {
                let parsed = &src[..pos];
                let result = (parsed, src, pos);

                Ok(result)
            }
        } else if byte.is('0') {
            let pos = pos + 1;
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        } else {
            let result = Error::Generic(f!("expected a digit, got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a digit, got none".to_string());

        Err(result)
    }
}

#[test]
fn test_natural() {
    assert!(natural(b"0", 0).is_ok());
    assert!(natural(b"1", 0).is_ok());

    assert!(natural(b"0123", 0).is_ok());
    assert!(natural(b"123", 0).is_ok());

    assert!(digits(b"_0123", 0).is_err());
    assert!(digits(b"123_", 0).is_err());

    assert!(natural(b"", 0).is_err());

    let case = natural(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = natural(b"0123", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = natural(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = natural(b"123", 0).unwrap();
    assert_eq!(case.0, b"123");

    let case = natural(b"0_123", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = natural(b"1_23", 0).unwrap();
    assert_eq!(case.0, b"1_23");

    let case = natural(b"123_123", 0).unwrap();
    assert_eq!(case.0, b"123_123");
}

/// <sign> ::= "-" | "+" | E
#[inline(always)]
pub fn sign(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('+') || byte.is('-') {
            let pos = pos + 1;
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        } else {
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }
}

#[test]
fn test_sign() {
    assert!(sign(b"+", 0).is_ok());
    assert!(sign(b"-", 0).is_ok());

    assert!(sign(b"", 0).is_ok());

    let case = sign(b"+", 0).unwrap();
    assert_eq!(case.0, b"+");

    let case = sign(b"-", 0).unwrap();
    assert_eq!(case.0, b"-");

    let case = sign(b"asdas", 0).unwrap();
    assert_eq!(case.0, b"");
}

/// <integer> ::= <sign> <natural>
#[inline(always)]
pub fn integer(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    let (_, src, pos) = sign(src, pos)?;
    natural(src, pos)
}

#[test]
fn test_integer() {
    assert!(integer(b"0", 0).is_ok());
    assert!(integer(b"1", 0).is_ok());

    assert!(integer(b"0123", 0).is_ok());
    assert!(integer(b"123", 0).is_ok());

    assert!(integer(b"+123", 0).is_ok());
    assert!(integer(b"-123", 0).is_ok());

    assert!(integer(b"-0", 0).is_ok());
    assert!(integer(b"+1", 0).is_ok());

    assert!(integer(b"+", 0).is_err());
    assert!(integer(b"-", 0).is_err());

    assert!(integer(b"_0123", 0).is_err());
    assert!(integer(b"123_", 0).is_err());

    assert!(integer(b"", 0).is_err());

    let case = integer(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = integer(b"0123", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = integer(b"0", 0).unwrap();
    assert_eq!(case.0, b"0");

    let case = integer(b"123", 0).unwrap();
    assert_eq!(case.0, b"123");

    let case = integer(b"-0_123", 0).unwrap();
    assert_eq!(case.0, b"-0");

    let case = integer(b"+1_23", 0).unwrap();
    assert_eq!(case.0, b"+1_23");

    let case = integer(b"+123_123", 0).unwrap();
    assert_eq!(case.0, b"+123_123");
}

/// <fraction> ::= E | "." <digits>
pub fn fraction(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('.') {
            digits(src, pos + 1)
        } else {
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }
}

#[test]
fn test_fraction() {
    assert!(fraction(b"0", 0).is_ok());
    assert!(fraction(b"1", 0).is_ok());

    assert!(fraction(b".0123", 0).is_ok());
    assert!(fraction(b".123", 0).is_ok());

    assert!(fraction(b".123", 0).is_ok());
    assert!(fraction(b".456", 0).is_ok());

    assert!(fraction(b".1_23", 0).is_ok());
    assert!(fraction(b".4_56", 0).is_ok());

    assert!(fraction(b"-0", 0).is_ok());
    assert!(fraction(b"+1", 0).is_ok());

    assert!(fraction(b"+", 0).is_ok());
    assert!(fraction(b"-", 0).is_ok());

    assert!(fraction(b"_0123", 0).is_ok());
    assert!(fraction(b"123_", 0).is_ok());

    assert!(fraction(b"", 0).is_ok());

    let case = fraction(b".1", 0).unwrap();
    assert_eq!(case.0, b".1");

    let case = fraction(b".123", 0).unwrap();
    assert_eq!(case.0, b".123");

    let case = fraction(b".0", 0).unwrap();
    assert_eq!(case.0, b".0");

    let case = fraction(b".123", 0).unwrap();
    assert_eq!(case.0, b".123");

    let case = fraction(b"+0", 0).unwrap();
    assert_eq!(case.0, b"");
}

/// <exponent> ::= E | "e" <sign> <digits> | "E" <sign> <digits>
pub fn exponent(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('e') || byte.is('E') {
            let pos = pos + 1;

            let (_, src, pos) = sign(src, pos)?;

            digits(src, pos)
        } else {
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }

    // if let Some(byte) = src.get(pos) {
    //     if byte.is('e') {
    //         let pos = pos + 1;

    //         if let Some(byte) = src.get(pos) {
    //             if byte.is('+') || byte.is('-') {
    //                 let pos = pos + 1;

    //                 digits(src, pos)
    //             } else {
    //                 digits(src, pos)
    //             }
    //         } else {
    //             let result = Error::Generic("expected a digit, '+', or '-', got none".to_string());

    //             Err(result)
    //         }
    //     } else {
    //         let result = Error::Generic(f!("expected a '.', got '{}'", byte.as_char()));

    //         Err(result)
    //     }
    // } else {
    //     let result = Error::Generic("expected a '.', got none".to_string());

    //     Err(result)
    // }
}

pub fn number(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    match integer(src, pos) {
        Ok((parsed, src, pos)) => match fraction(src, pos) {
            Ok((parsed, src, pos)) => match exponent(src, pos) {
                Ok(result) => Ok(result),
                Err(_) => {
                    let result = (parsed, src, pos);

                    Ok(result)
                }
            },
            Err(_) => {
                let result = (parsed, src, pos);

                Ok(result)
            }
        },

        Err(result) => Err(result),
    }
}

#[test]
fn test_number() {
    assert!(number(b"123", 0).is_ok());
    assert!(number(b"123a", 0).is_ok());
    assert!(number(b"0", 0).is_ok());
    assert!(number(b"1", 0).is_ok());
    assert!(number(b"+0", 0).is_ok());
    assert!(number(b"-1", 0).is_ok());
    assert!(number(b"a", 0).is_err());
    assert!(number(b"+", 0).is_err());
    assert!(number(b"-", 0).is_err());
    assert!(number(b"", 0).is_err());
    assert!(number(b"123.123", 0).is_ok());
    assert!(number(b"123_123.123", 0).is_ok());
    assert!(number(b"0.1", 0).is_ok());
    assert!(number(b"1.2", 0).is_ok());
    assert!(number(b"+0.0", 0).is_ok());
    assert!(number(b"-1.0", 0).is_ok());

    assert!(number(b"123_123.21_", 0).is_ok());
    assert!(number(b"_123", 0).is_err());

    let case = number(b"123.1", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 5);

    let case = number(b"0.12", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 4);

    let case = number(b"1.0", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 3);

    let case = number(b"123.34", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 6);

    let case = number(b"-0.12", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 5);

    let case = number(b"-23.0", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 5);

    let case = number(b"123", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 3);

    let case = number(b"012", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"1", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"0", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"-0", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 2);

    let case = number(b"-23", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 3);

    let case = number(b"123", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 3);

    let case = number(b"012", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"1", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"0", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 1);

    let case = number(b"123e1", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 5);

    let case = number(b"123e+1", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 6);

    let case = number(b"123e-123_311", 0).unwrap();

    let position = case.2;

    assert_eq!(position, 12);
}
