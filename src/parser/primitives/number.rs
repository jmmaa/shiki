use crate::prelude::*;

use crate::utils::ByteUtil;

use tailcall::tailcall;

/*

<number> ::= <integer> | <float> | <exponent>

<float> ::= <integer> "." <digits>

<exponent> ::= <integer> "e" "+" <digits> |
               <integer> "e" "-" <digits> |
               <integer> "e" <digits>     |
               <float> "e" "+" <digits>   |
               <float> "e" "-" <digits>   |
               <float> "e" <digits>

<integer> ::= "+" <natural> | "-" <natural> | <natural>

<natural> ::= <nonzero> <digits> | <nonzero> "_" <digits> | <digit>

<digits> ::=  <digit> <digits> | <digit> "_" <digits> | <digit>

<digit> ::= "0"  | <nonzero>

<nonzero> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"


*/

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

#[inline(always)]
pub fn integer(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('+') || byte.is('-') {
            let pos = pos + 1;
            natural(src, pos)
        } else {
            natural(src, pos)
        }
    } else {
        let result = Error::Generic("expected a digit, '+', or '-', got none".to_string());

        Err(result)
    }
}

pub fn float(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    let (_, src, pos) = integer(src, pos)?;

    if let Some(byte) = src.get(pos) {
        if byte.is('.') {
            digits(src, pos + 1)
        } else {
            let result = Error::Generic(f!("expected a '.', got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a '.', got none".to_string());

        Err(result)
    }
}

pub fn exponent(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    let (_, src, pos) = float(src, pos).or(integer(src, pos))?;

    if let Some(byte) = src.get(pos) {
        if byte.is('e') {
            let pos = pos + 1;

            if let Some(byte) = src.get(pos) {
                if byte.is('+') || byte.is('-') {
                    let pos = pos + 1;

                    digits(src, pos)
                } else {
                    digits(src, pos)
                }
            } else {
                let result = Error::Generic("expected a digit, '+', or '-', got none".to_string());

                Err(result)
            }
        } else {
            let result = Error::Generic(f!("expected a '.', got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a '.', got none".to_string());

        Err(result)
    }
}

pub fn number(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    exponent(src, pos).or(float(src, pos)).or(integer(src, pos))
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
