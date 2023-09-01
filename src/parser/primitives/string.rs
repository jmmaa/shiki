use super::prelude::*;

use super::utils::ByteUtil;

use tailcall::tailcall;

#[tailcall]
fn inline_string_chars(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('"') {
            let pos = pos + 1;

            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        } else if byte.is('\\') {
            let pos = pos + 1;

            let (_, src, pos) = resolve_escape_sequence(src, pos)?;

            inline_string_chars(src, pos)
        } else if !byte.is_ascii_control() {
            inline_string_chars(src, pos + 1)
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

pub fn inline_string(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('"') {
            inline_string_chars(src, pos + 1)
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
    assert!(inline_string(r#""Hello, World!""#.as_bytes(), 0).is_ok());
    assert!(inline_string(r#""Hello, World!""#.as_bytes(), 0).is_ok());
    assert!(inline_string(
        r#""
        Hello, World!""#
            .as_bytes(),
        0
    )
    .is_err());

    assert!(inline_string(r#""""#.as_bytes(), 0).is_ok());
    assert!(inline_string(r#"""#.as_bytes(), 0).is_err());
    assert!(inline_string(r#""#.as_bytes(), 0).is_err());

    assert!(inline_string(r#""\n""#.as_bytes(), 0).is_ok());
    assert!(inline_string(r#""\uFFFF""#.as_bytes(), 0).is_ok());
}

fn resolve_escape_sequence(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('"')
            || byte.is('\\')
            || byte.is('/')
            || byte.is('b')
            || byte.is('f')
            || byte.is('n')
            || byte.is('r')
            || byte.is('t')
        {
            let pos = pos + 1;
            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        } else if byte.is('u') {
            let pos = pos + 1;

            resolve_hex_digits(src, pos)
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
    assert!(resolve_escape_sequence(b"\"", 0).is_ok());
    assert!(resolve_escape_sequence(b"\\", 0).is_ok());
    assert!(resolve_escape_sequence(b"/", 0).is_ok());
    assert!(resolve_escape_sequence(b"b", 0).is_ok());
    assert!(resolve_escape_sequence(b"f", 0).is_ok());
    assert!(resolve_escape_sequence(b"n", 0).is_ok());
    assert!(resolve_escape_sequence(b"r", 0).is_ok());
    assert!(resolve_escape_sequence(b"t", 0).is_ok());

    assert!(resolve_escape_sequence(b"uFFFF", 0).is_ok());

    assert!(resolve_escape_sequence(b"z", 0).is_err());
    assert!(resolve_escape_sequence(b"u", 0).is_err());
}

#[tailcall]
fn hexdigits(len: usize, src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if len != 0 {
        if let Some(byte) = src.get(pos) {
            if byte.is_ascii_hexdigit() {
                hexdigits(len - 1, src, pos + 1)
            } else {
                let result = Error::Generic(f!("expected a hex digit, got '{}'", byte.as_char()));

                Err(result)
            }
        } else {
            let result = Error::Generic("expected a hex digit, got none".to_string());

            Err(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }
}

fn resolve_hex_digits(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    hexdigits(4, src, pos)
}

#[test]
fn test_resolve_hex_digits() {
    assert!(resolve_hex_digits(b"FFFF", 0).is_ok());
    assert!(resolve_hex_digits(b"12AF", 0).is_ok());
    assert!(resolve_hex_digits(b"45F2", 0).is_ok());
    assert!(resolve_hex_digits(b"FFF", 0).is_err());
    assert!(resolve_hex_digits(b"A", 0).is_err());
    assert!(resolve_hex_digits(b"A2", 0).is_err());
    assert!(resolve_hex_digits(b"", 0).is_err());
}

#[tailcall]
fn quotes(len: usize, src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if len != 0 {
        if let Some(byte) = src.get(pos) {
            if byte.is('"') {
                quotes(len - 1, src, pos + 1)
            } else {
                let result = Error::Generic(f!("expected a '\"', got '{}'", byte.as_char()));

                Err(result)
            }
        } else {
            let result = Error::Generic("expected a '\"', got none".to_string());

            Err(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }
}

pub fn multiline_string_delimiter(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    quotes(3, src, pos)
}

#[tailcall]
fn multiline_string_chars(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('"') {
            let (_, src, pos) = multiline_string_delimiter(src, pos)?;

            let parsed = &src[..pos];
            let result = (parsed, src, pos);

            Ok(result)
        } else if byte.is('\\') {
            let pos = pos + 1;

            let (_, src, pos) = resolve_escape_sequence(src, pos)?;

            multiline_string_chars(src, pos)
        } else {
            multiline_string_chars(src, pos + 1)
        }
    } else {
        let result = Error::Generic("expected a character, '\"', or '\\', got none".to_string());

        Err(result)
    }
}

pub fn multiline_string(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    if let Some(byte) = src.get(pos) {
        if byte.is('"') {
            let (_, src, pos) = multiline_string_delimiter(src, pos)?;

            multiline_string_chars(src, pos)
        } else {
            let result = Error::Generic(f!("expected a '\"', got '{}'", byte.as_char()));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected a '\"', got none".to_string());

        Err(result)
    }
}
