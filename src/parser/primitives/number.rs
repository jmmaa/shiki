use super::prelude::*;

use super::utils::ByteUtil;

use tailcall::tailcall;

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
        let result = Error::Generic("expected an integer, got none".to_string());

        Err(result)
    }
}

#[inline(always)]
fn resolve_float(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    digits(src, pos)
}

#[inline(always)]
fn resolve_exponent(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
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
}

pub fn number(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    let (_, src, pos) = integer(src, pos)?;

    if let Some(byte) = src.get(pos) {
        if byte.is('.') {
            let pos = pos + 1;

            let (_, src, pos) = resolve_float(src, pos)?;

            if let Some(b'e') = src.get(pos) {
                let pos = pos + 1;

                resolve_exponent(src, pos)
            } else {
                let parsed = &src[..pos];
                let result = (parsed, src, pos);

                Ok(result)
            }
        } else if byte.is('e') {
            let pos = pos + 1;

            resolve_exponent(src, pos)
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

    assert!(number(b"123_123.21_", 0).is_err());
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
