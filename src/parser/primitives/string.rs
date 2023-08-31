use super::prelude::*;

use super::context::Context;
use super::utils::ByteUtil;

use tailcall::tailcall;

#[tailcall]
fn recurse_inline_string_chars(source: &[u8], position: usize) -> Result<Context> {
    if let Some(byte) = source.get(position) {
        if byte.is('"') {
            let result = Context::new(source, position + 1);

            Ok(result)
        } else if byte.is('\\') {
            let ctx = Context::new(source, position + 1);

            let (_, ctx) = escape_sequence(ctx)?;

            recurse_inline_string_chars(ctx.source(), ctx.position())
        } else if !byte.is_ascii_control() {
            recurse_inline_string_chars(source, position + 1)
        } else {
            let result = Error::Generic(f!(
                "expected a non-control character, '\"', or '\\' , got '{}'",
                byte.as_char()
            ));

            Err(result)
        }
    } else {
        let result =
            Error::Generic("expected a non-control character, '\"', or '\\', got none".to_string());

        Err(result)
    }
}

pub fn inline_string(ctx: Context) -> Result<(&[u8], Context)> {
    if let Some(byte) = ctx.get_current_byte() {
        if byte.is('"') {
            let result = recurse_inline_string_chars(ctx.source(), ctx.position() + 1);

            result.map(|ctx| (ctx.get_current_slice(), ctx))
        } else {
            let result = Error::Generic(f!("expected a '\"', got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a '\"', got none".to_string());

        Err(result)
    }
}

#[test]
fn test_inline_string() {
    assert!(inline_string(Context::new(r#""Hello, World!""#.as_bytes(), 0)).is_ok());
    assert!(inline_string(Context::new(r#""Hello, World!""#.as_bytes(), 0)).is_ok());
    assert!(inline_string(Context::new(
        r#""
        Hello, World!""#
            .as_bytes(),
        0
    ))
    .is_err());

    assert!(inline_string(Context::new(r#""""#.as_bytes(), 0)).is_ok());
    assert!(inline_string(Context::new(r#"""#.as_bytes(), 0)).is_err());
    assert!(inline_string(Context::new(r#""#.as_bytes(), 0)).is_err());

    assert!(inline_string(Context::new(r#""\n""#.as_bytes(), 0)).is_ok());
    assert!(inline_string(Context::new(r#""\uFFFF""#.as_bytes(), 0)).is_ok());
}

fn escape_sequence(ctx: Context) -> Result<(&[u8], Context)> {
    if let Some(byte) = ctx.get_current_byte() {
        if byte.is('"')
            || byte.is('\\')
            || byte.is('/')
            || byte.is('b')
            || byte.is('f')
            || byte.is('n')
            || byte.is('r')
            || byte.is('t')
        {
            let ctx = Context::new(ctx.source(), ctx.position() + 1);
            let result = (ctx.get_current_slice(), ctx);

            Ok(result)
        } else if byte.is('u') {
            let ctx = Context::new(ctx.source(), ctx.position() + 1);

            resolve_hex_digits(ctx)
        } else {
            let result =
                Error::Generic(f!("expected an escape sequence, got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected an escape sequence, got none".to_string());

        Err(result)
    }
}

#[test]
fn test_escape_sequence() {
    assert!(escape_sequence(Context::new(b"\"", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"\\", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"/", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"b", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"f", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"n", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"r", 0)).is_ok());
    assert!(escape_sequence(Context::new(b"t", 0)).is_ok());

    assert!(escape_sequence(Context::new(b"uFFFF", 0)).is_ok());

    assert!(escape_sequence(Context::new(b"z", 0)).is_err());
    assert!(escape_sequence(Context::new(b"u", 0)).is_err());
}

#[tailcall]
fn recurse_hexdigits(length: usize, source: &[u8], position: usize) -> Result<Context> {
    if length != 0 {
        if let Some(byte) = source.get(position) {
            if byte.is_ascii_hexdigit() {
                let length = length - 1;
                let position = position + 1;

                recurse_hexdigits(length, source, position)
            } else {
                let result = Error::Generic(f!("expected a hex digit, got '{}'", byte.as_char()));

                Err(result)
            }
        } else {
            let result = Error::Generic("expected a hex digit, got none".to_string());

            Err(result)
        }
    } else {
        let result = Context::new(source, position);

        Ok(result)
    }
}

fn resolve_hex_digits(ctx: Context) -> Result<(&[u8], Context)> {
    let result = recurse_hexdigits(4, ctx.source(), ctx.position());

    result.map(|ctx| (ctx.get_current_slice(), ctx))
}

#[test]
fn test_resolve_hex_digits() {
    assert!(resolve_hex_digits(Context::new(b"FFFF", 0)).is_ok());
    assert!(resolve_hex_digits(Context::new(b"12AF", 0)).is_ok());
    assert!(resolve_hex_digits(Context::new(b"45F2", 0)).is_ok());
    assert!(resolve_hex_digits(Context::new(b"FFF", 0)).is_err());
    assert!(resolve_hex_digits(Context::new(b"A", 0)).is_err());
    assert!(resolve_hex_digits(Context::new(b"A2", 0)).is_err());
    assert!(resolve_hex_digits(Context::new(b"", 0)).is_err());
}
