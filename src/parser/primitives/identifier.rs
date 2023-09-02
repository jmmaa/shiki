use crate::prelude::*;

use crate::utils::ByteUtil;

use tailcall::tailcall;

#[tailcall]
pub fn alphanumerics(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is_ascii_alphanumeric() {
            let pos = pos + 1;

            if let Some(byte) = src.get(pos) {
                if byte.is_ascii_alphanumeric() {
                    alphanumerics(src, pos)
                } else if byte.is('_') {
                    alphanumerics(src, pos + 1)
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
            let result = Error::Generic(f!("expected a letter or digit, got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a letter or digit, got none".to_string());

        Err(result)
    }
}

pub fn identifier(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is_ascii_alphabetic() {
            let pos = pos + 1;

            if let Some(byte) = src.get(pos) {
                if byte.is_ascii_alphanumeric() {
                    alphanumerics(src, pos)
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
            let result = Error::Generic(f!("expected a letter, got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a digit, got none".to_string());

        Err(result)
    }
}

#[test]
fn test_identifier() {
    assert!(identifier(b"123", 0).is_err());
    assert!(identifier(b"abcd", 0).is_ok());
    assert!(identifier(b"abcd123", 0).is_ok());
    assert!(identifier(b"123abcd", 0).is_err());
    assert!(identifier(b"abcd_123", 0).is_ok());
    assert!(identifier(b"-", 0).is_err());
    assert!(identifier(b"_", 0).is_err());

    let case = identifier(b"a123", 0).unwrap();

    let pos = case.2;

    assert_eq!(pos, 4);

    let case = identifier(b"asd123", 0).unwrap();

    let pos = case.2;

    assert_eq!(pos, 6);

    let case = identifier(b"a123_123", 0).unwrap();

    let pos = case.2;

    assert_eq!(pos, 8);

    let case = identifier(b"asdas_asd", 0).unwrap();

    let pos = case.2;

    assert_eq!(pos, 9);

    let case = identifier(b"asd123_23asd", 0).unwrap();

    let pos = case.2;

    assert_eq!(pos, 12);
}
