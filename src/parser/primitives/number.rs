use super::prelude::*;

use super::context::Context;

use super::utils::ByteUtil;

use tailcall::tailcall;

#[tailcall]
fn digits_split_position(source: &[u8], position: usize) -> Result<Context> {
    match source.get(position) {
        Some(byte) => {
            if byte.is_ascii_digit() {
                let position = position + 1;

                if let Some(byte) = source.get(position) {
                    if byte.is_ascii_digit() {
                        digits_split_position(source, position)
                    } else if byte.is_ascii_underscore() {
                        digits_split_position(source, position + 1)
                    } else {
                        let result = Context::new(source, position);

                        Ok(result)
                    }
                } else {
                    let result = Context::new(source, position);

                    Ok(result)
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
pub fn digits(ctx: Context) -> Result<(&[u8], Context)> {
    let result = digits_split_position(ctx.source(), ctx.position());

    result.map(|ctx| (ctx.get_current_slice(), ctx))
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
pub fn natural(ctx: Context) -> Result<(&[u8], Context)> {
    match ctx.get_current_byte() {
        Some(byte) => {
            if byte.is_ascii_nonzero_digit() {
                let ctx = Context::new(ctx.source(), ctx.position() + 1);

                if let Some(byte) = ctx.get_current_byte() {
                    if byte.is_ascii_digit() {
                        digits(ctx)
                    } else {
                        let parsed = ctx.get_current_slice();
                        let result = (parsed, ctx);

                        Ok(result)
                    }
                } else {
                    let parsed = ctx.get_current_slice();
                    let result = (parsed, ctx);

                    Ok(result)
                }
            } else if byte.is_ascii_zero_digit() {
                let ctx = Context::new(ctx.source(), ctx.position() + 1);
                let parsed = ctx.get_current_slice();
                let result = (parsed, ctx);

                Ok(result)
            } else {
                let result = Error::Generic(f!("expected a digit, got '{}'", byte.as_char()));

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
pub fn integer(ctx: Context) -> Result<(&[u8], Context)> {
    if let Some(byte) = ctx.get_current_byte() {
        if byte.is_ascii_minus() || byte.is_ascii_plus() {
            let ctx = Context::new(ctx.source(), ctx.position() + 1);
            natural(ctx)
        } else {
            natural(ctx)
        }
    } else {
        let result = Error::Generic("expected an integer, got none".to_string());

        Err(result)
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

// pub fn float(ctx: Context) -> Result<(&[u8], Context)> {
//     match integer(ctx) {
//         Ok((_, ctx)) => {
//             if let Some(byte) = ctx.get_current_byte() {
//                 if byte.is_ascii_period() {
//                     let ctx = Context::new(ctx.source(), ctx.position() + 1);

//                     digits(ctx)
//                 } else {
//                     let result = Error::Generic(f!("expected a '.', got '{}'", byte.as_char()));

//                     Err(result)
//                 }
//             } else {
//                 let result = Error::Generic("expected a '.', got none".to_string());

//                 Err(result)
//             }
//         }
//         Err(result) => Err(result),
//     }
// }

// #[test]
// fn test_float() {
//     assert!(float(Context::new(b"123", 0)).is_err());
//     assert!(float(Context::new(b"123a", 0)).is_err());
//     assert!(float(Context::new(b"0", 0)).is_err());
//     assert!(float(Context::new(b"1", 0)).is_err());
//     assert!(float(Context::new(b"+0", 0)).is_err());
//     assert!(float(Context::new(b"-1", 0)).is_err());
//     assert!(float(Context::new(b"a", 0)).is_err());
//     assert!(float(Context::new(b"+", 0)).is_err());
//     assert!(float(Context::new(b"-", 0)).is_err());
//     assert!(float(Context::new(b"", 0)).is_err());
//     assert!(float(Context::new(b"123.123", 0)).is_ok());
//     assert!(float(Context::new(b"123_123.123", 0)).is_ok());
//     assert!(float(Context::new(b"0.1", 0)).is_ok());
//     assert!(float(Context::new(b"1.2", 0)).is_ok());
//     assert!(float(Context::new(b"+0.0", 0)).is_ok());
//     assert!(float(Context::new(b"-1.0", 0)).is_ok());

//     assert!(float(Context::new(b"123_123.21_", 0)).is_err());
//     assert!(float(Context::new(b"_123", 0)).is_err());

//     let case = float(Context::new(b"123.1", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 5);

//     let case = float(Context::new(b"0.12", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 4);

//     let case = float(Context::new(b"1.0", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 3);

//     let case = float(Context::new(b"123.34", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 6);

//     let case = float(Context::new(b"-0.12", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 5);

//     let case = float(Context::new(b"-23.0", 0)).unwrap();

//     let position = case.1.position();

//     assert_eq!(position, 5);
// }

fn resolve_float(ctx: Context) -> Result<(&[u8], Context)> {
    digits(ctx)
}

fn resolve_exponent(ctx: Context) -> Result<(&[u8], Context)> {
    if let Some(byte) = ctx.get_current_byte() {
        if byte.is_ascii_plus() || byte.is_ascii_minus() {
            let ctx = Context::new(ctx.source(), ctx.position() + 1);

            digits(ctx)
        } else {
            digits(ctx)
        }
    } else {
        let result = Error::Generic("expected a digit, '+', or '-', got none".to_string());

        Err(result)
    }
}

// REFACTOR THIS
pub fn number(ctx: Context) -> Result<(&[u8], Context)> {
    match integer(ctx) {
        Ok((parsed, ctx)) => {
            if let Some(byte) = ctx.get_current_byte() {
                if byte.is_ascii_period() {
                    let ctx = Context::new(ctx.source(), ctx.position() + 1);

                    match resolve_float(ctx) {
                        Ok((parsed, ctx)) => {
                            if let Some(byte) = ctx.get_current_byte() {
                                if byte.as_char() == 'e' {
                                    let ctx = Context::new(ctx.source(), ctx.position() + 1);

                                    resolve_exponent(ctx)
                                } else {
                                    let result = (parsed, ctx);

                                    Ok(result)
                                }
                            } else {
                                let result = (parsed, ctx);

                                Ok(result)
                            }
                        }
                        Err(result) => Err(result),
                    }
                } else if byte.as_char() == 'e' {
                    let ctx = Context::new(ctx.source(), ctx.position() + 1);

                    resolve_exponent(ctx)
                } else {
                    let result = (parsed, ctx);

                    Ok(result)
                }
            } else {
                let result = (parsed, ctx);

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
