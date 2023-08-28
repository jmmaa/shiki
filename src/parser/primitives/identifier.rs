use super::prelude::*;

use super::context::Context;
use super::utils::ByteUtil;

use tailcall::tailcall;

#[tailcall]
pub fn alphanumerics_split_position(source: &[u8], position: usize) -> Result<Context> {
    if let Some(&byte) = source.get(position) {
        if byte.is_ascii_alphanumeric() {
            let position = position + 1;

            match source.get(position) {
                Some(&byte) => {
                    if byte.is_ascii_alphanumeric() {
                        alphanumerics_split_position(source, position)
                    } else if byte.is_ascii_underscore() {
                        alphanumerics_split_position(source, position + 1)
                    } else {
                        let result = Context::new(source, position);

                        Ok(result)
                    }
                }
                None => {
                    let result = Context::new(source, position);

                    Ok(result)
                }
            }
        } else {
            let result = Error::Generic(f!("expected a letter or digit, got '{}'", byte as char));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a letter or digit, got none".to_string());

        Err(result)
    }
}

#[test]

fn test_alphanumerics_split_position() {
    assert!(alphanumerics_split_position(b"123", 0).is_ok());
    assert!(alphanumerics_split_position(b"abcd", 0).is_ok());
    assert!(alphanumerics_split_position(b"abcd123", 0).is_ok());
    assert!(alphanumerics_split_position(b"123abcd", 0).is_ok());
    assert!(alphanumerics_split_position(b"abcd_123", 0).is_ok());
    assert!(alphanumerics_split_position(b"-", 0).is_err());
}

#[inline(always)]
pub fn alphanumerics(ctx: Context) -> Result<(&[u8], Context)> {
    let result = alphanumerics_split_position(ctx.source(), ctx.position());

    result.map(|ctx| (ctx.get_current_slice(), ctx))
}

#[test]

fn test_alphanumerics() {
    assert!(alphanumerics(Context::new(b"123", 0)).is_ok());
    assert!(alphanumerics(Context::new(b"abcd", 0)).is_ok());
    assert!(alphanumerics(Context::new(b"abcd123", 0)).is_ok());
    assert!(alphanumerics(Context::new(b"123abcd", 0)).is_ok());
    assert!(alphanumerics(Context::new(b"abcd_123", 0)).is_ok());
    assert!(alphanumerics(Context::new(b"-", 0)).is_err());

    let case = alphanumerics(Context::new(b"123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 3);

    let case = alphanumerics(Context::new(b"asd123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 6);

    let case = alphanumerics(Context::new(b"123_123", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 7);

    let case = alphanumerics(Context::new(b"asdas_asd", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 9);

    let case = alphanumerics(Context::new(b"asd123_23asd", 0)).unwrap();

    let position = case.1.position();

    assert_eq!(position, 12);
}
