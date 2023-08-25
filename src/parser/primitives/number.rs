use super::prelude::*;

use super::context::Context;

use super::utils::{ByteSliceUtil, ByteUtil};

use tailcall::tailcall;

#[tailcall]
fn digits_split_position(source: &'static [u8], position: usize) -> Result<usize> {
    match source.get(position) {
        Some(byte) => {
            if byte.is_ascii_digit() {
                let position = position + 1;

                match source.get(position) {
                    Some(byte) => {
                        if byte.is_ascii_digit() {
                            digits_split_position(source, position)
                        } else if byte.is_ascii_underscore() {
                            digits_split_position(source, position + 1)
                        } else {
                            let result = position;

                            Ok(result)
                        }
                    }
                    None => {
                        let result = position;

                        Ok(result)
                    }
                }
            } else {
                let result = Error::Generic(f!("expected a digit, got '{}'", *byte as char));

                Err(result)
            }
        }

        None => {
            let result = Error::Generic("expected a digit, got none".to_string());

            Err(result)
        }
    }
}

#[inline(always)]
pub fn digits(context: Context) -> Result<(&'static [u8], Context)> {
    let source = context.source();
    let position = context.position();

    let result = digits_split_position(source, position);

    result.map(|position| {
        let parsed = source.slice_until(position);

        let context = Context::new(source, position);

        (parsed, context)
    })
}

#[inline(always)]
pub fn digits0(context: Context) -> Result<(&'static [u8], Context)> {
    let source = context.source();
    let position = context.position();

    match source.get(position) {
        Some(byte) => {
            if byte.is_ascii_digit() {
                let result = digits_split_position(source, position);

                result.map(|position| {
                    let parsed = source.slice_until(position);

                    let context = Context::new(source, position);

                    (parsed, context)
                })
            } else {
                let parsed = source.slice_until(position);
                let context = Context::new(source, position);

                let result = (parsed, context);

                Ok(result)
            }
        }
        None => {
            let parsed = source.slice_until(position);
            let context = Context::new(source, position);

            let result = (parsed, context);

            Ok(result)
        }
    }
}

#[test]
fn test_digits() {
    assert!(digits(Context::new(b"123", 0)).is_ok());
    assert!(digits(Context::new(b"123a", 0)).is_ok());
    assert!(digits(Context::new(b"0", 0)).is_ok());
    assert!(digits(Context::new(b"1", 0)).is_ok());
    assert!(digits(Context::new(b"a", 0)).is_err());
    assert!(digits(Context::new(b"", 0)).is_err());

    assert!(digits(Context::new(b"123_", 0)).is_err());
    assert!(digits(Context::new(b"_123", 0)).is_err());

    let case = digits(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = digits(Context::new(b"012", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = digits(Context::new(b"1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = digits(Context::new(b"0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);
}

#[inline(always)]
pub fn natural(context: Context) -> Result<(&'static [u8], Context)> {
    let source = context.source();
    let position = context.position();

    match source.get(position) {
        Some(byte) => {
            if byte.is_ascii_nonzero_digit() {
                let position = position + 1;

                match source.get(position) {
                    Some(byte) => {
                        if byte.is_ascii_digit() {
                            digits(Context::new(source, position))
                        } else if byte == &b'0' {
                            digits(Context::new(source, position + 1))
                        } else {
                            let parsed = source.slice_until(position);
                            let context = Context::new(source, position);
                            let result = (parsed, context);

                            Ok(result)
                        }
                    }
                    None => {
                        let parsed = source.slice_until(position);
                        let context = Context::new(source, position);
                        let result = (parsed, context);

                        Ok(result)
                    }
                }
            } else if byte.is_ascii_zero_digit() {
                let position = position + 1;

                let parsed = source.slice_until(position);
                let context = Context::new(source, position);
                let result = (parsed, context);

                Ok(result)
            } else {
                let result = Error::Generic(f!("expected a digit, got '{}'", *byte as char));

                Err(result)
            }
        }
        None => {
            let result = Error::Generic("expected a digit, got none".to_string());

            Err(result)
        }
    }
}

#[test]
fn test_natural() {
    assert!(natural(Context::new(b"123", 0)).is_ok());
    assert!(natural(Context::new(b"123a", 0)).is_ok());
    assert!(natural(Context::new(b"0", 0)).is_ok());
    assert!(natural(Context::new(b"1", 0)).is_ok());
    assert!(natural(Context::new(b"a", 0)).is_err());
    assert!(natural(Context::new(b"", 0)).is_err());

    assert!(natural(Context::new(b"123_", 0)).is_err());
    assert!(natural(Context::new(b"_123", 0)).is_err());

    let case = natural(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = natural(Context::new(b"012", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = natural(Context::new(b"1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = natural(Context::new(b"0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);
}

#[inline(always)]
pub fn integer(context: Context) -> Result<(&'static [u8], Context)> {
    let source = context.source();
    let position = context.position();

    match source.get(position) {
        Some(byte) => {
            if byte.is_ascii_minus() || byte.is_ascii_plus() {
                natural(Context::new(source, position + 1))
            } else {
                natural(Context::new(source, position))
            }
        }

        None => {
            let result = Error::Generic("expected an integer, got none".to_string());

            Err(result)
        }
    }
}

#[test]
fn test_integer() {
    assert!(integer(Context::new(b"123", 0)).is_ok());
    assert!(integer(Context::new(b"123a", 0)).is_ok());
    assert!(integer(Context::new(b"0", 0)).is_ok());
    assert!(integer(Context::new(b"1", 0)).is_ok());
    assert!(integer(Context::new(b"+0", 0)).is_ok());
    assert!(integer(Context::new(b"-1", 0)).is_ok());
    assert!(integer(Context::new(b"a", 0)).is_err());
    assert!(integer(Context::new(b"+", 0)).is_err());
    assert!(integer(Context::new(b"-", 0)).is_err());
    assert!(integer(Context::new(b"", 0)).is_err());

    assert!(integer(Context::new(b"123_", 0)).is_err());
    assert!(integer(Context::new(b"_123", 0)).is_err());

    let case = integer(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = integer(Context::new(b"012", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = integer(Context::new(b"1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = integer(Context::new(b"0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = integer(Context::new(b"-0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 2);

    let case = integer(Context::new(b"-23", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);
}

pub fn float(context: Context) -> Result<(&'static [u8], Context)> {
    match integer(context) {
        Ok(result) => {
            let context = result.1;

            let source = context.source();
            let position = context.position();

            if let Some(byte) = source.get(position) {
                if byte.is_ascii_period() {
                    let context = Context::new(source, position + 1);

                    digits(context)
                } else {
                    let result = Error::Generic(f!("expected a '.', got '{}'", byte.as_char()));

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

#[test]
fn test_float() {
    assert!(float(Context::new(b"123", 0)).is_err());
    assert!(float(Context::new(b"123a", 0)).is_err());
    assert!(float(Context::new(b"0", 0)).is_err());
    assert!(float(Context::new(b"1", 0)).is_err());
    assert!(float(Context::new(b"+0", 0)).is_err());
    assert!(float(Context::new(b"-1", 0)).is_err());
    assert!(float(Context::new(b"a", 0)).is_err());
    assert!(float(Context::new(b"+", 0)).is_err());
    assert!(float(Context::new(b"-", 0)).is_err());
    assert!(float(Context::new(b"", 0)).is_err());
    assert!(float(Context::new(b"123.123", 0)).is_ok());
    assert!(float(Context::new(b"123_123.123", 0)).is_ok());
    assert!(float(Context::new(b"0.1", 0)).is_ok());
    assert!(float(Context::new(b"1.2", 0)).is_ok());
    assert!(float(Context::new(b"+0.0", 0)).is_ok());
    assert!(float(Context::new(b"-1.0", 0)).is_ok());

    assert!(float(Context::new(b"123_123.21_", 0)).is_err());
    assert!(float(Context::new(b"_123", 0)).is_err());

    let case = float(Context::new(b"123.1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = float(Context::new(b"0.12", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 4);

    let case = float(Context::new(b"1.0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = float(Context::new(b"123.34", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 6);

    let case = float(Context::new(b"-0.12", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = float(Context::new(b"-23.0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);
}

// REFACTOR THIS
pub fn number(context: Context) -> Result<(&'static [u8], Context)> {
    match integer(context) {
        Ok(result) => {
            let (parsed, context) = result;

            let source = context.source();
            let position = context.position();

            if let Some(byte) = source.get(position) {
                if byte.is_ascii_period() {
                    match digits(Context::new(source, position + 1)) {
                        Ok(result) => {
                            let (_, context) = result;

                            let position = context.position();

                            match source.get(position) {
                                Some(byte) => {
                                    if byte.as_char() == 'e' {
                                        let position = position + 1;

                                        match source.get(position) {
                                            Some(byte) => {
                                                if byte.is_ascii_plus() || byte.is_ascii_minus() {
                                                    let context =
                                                        Context::new(source, position + 1);

                                                    digits(context)
                                                } else {
                                                    let context = Context::new(source, position);

                                                    digits(context)
                                                }
                                            }
                                            None => {
                                                let result = Error::Generic(
                                                    "expected a digit, '+', or '-', got none"
                                                        .to_string(),
                                                );

                                                Err(result)
                                            }
                                        }
                                    } else {
                                        let result = (parsed, context);

                                        Ok(result)
                                    }
                                }
                                None => {
                                    let result = (parsed, context);

                                    Ok(result)
                                }
                            }
                        }

                        Err(result) => Err(result),
                    }
                } else if byte.as_char() == 'e' {
                    let position = position + 1;

                    match source.get(position) {
                        Some(byte) => {
                            if byte.is_ascii_plus() || byte.is_ascii_minus() {
                                let context = Context::new(source, position + 1);

                                digits(context)
                            } else {
                                let context = Context::new(source, position);

                                digits(context)
                            }
                        }
                        None => {
                            let result = Error::Generic(
                                "expected a digit, '+', or '-', got none".to_string(),
                            );

                            Err(result)
                        }
                    }
                } else {
                    let result = (parsed, context);

                    Ok(result)
                }
            } else {
                let result = (parsed, context);

                Ok(result)
            }
        }
        Err(result) => Err(result),
    }
}

#[test]
fn test_number() {
    assert!(number(Context::new(b"123", 0)).is_ok());
    assert!(number(Context::new(b"123a", 0)).is_ok());
    assert!(number(Context::new(b"0", 0)).is_ok());
    assert!(number(Context::new(b"1", 0)).is_ok());
    assert!(number(Context::new(b"+0", 0)).is_ok());
    assert!(number(Context::new(b"-1", 0)).is_ok());
    assert!(number(Context::new(b"a", 0)).is_err());
    assert!(number(Context::new(b"+", 0)).is_err());
    assert!(number(Context::new(b"-", 0)).is_err());
    assert!(number(Context::new(b"", 0)).is_err());
    assert!(number(Context::new(b"123.123", 0)).is_ok());
    assert!(number(Context::new(b"123_123.123", 0)).is_ok());
    assert!(number(Context::new(b"0.1", 0)).is_ok());
    assert!(number(Context::new(b"1.2", 0)).is_ok());
    assert!(number(Context::new(b"+0.0", 0)).is_ok());
    assert!(number(Context::new(b"-1.0", 0)).is_ok());

    assert!(number(Context::new(b"123_123.21_", 0)).is_err());
    assert!(number(Context::new(b"_123", 0)).is_err());

    let case = number(Context::new(b"123.1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = number(Context::new(b"0.12", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 4);

    let case = number(Context::new(b"1.0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = number(Context::new(b"123.34", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 6);

    let case = number(Context::new(b"-0.12", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = number(Context::new(b"-23.0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = number(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = number(Context::new(b"012", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"-0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 2);

    let case = number(Context::new(b"-23", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = number(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = number(Context::new(b"012", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"0", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 1);

    let case = number(Context::new(b"123e1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 5);

    let case = number(Context::new(b"123e+1", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 6);

    let case = number(Context::new(b"123e-123_311", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 12);
}
