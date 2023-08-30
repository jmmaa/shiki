use super::prelude::*;

use super::context::Context;
use super::utils::ByteUtil;

use tailcall::tailcall;

pub enum StringParseState {
    Starting,
    Consuming,
    Ending,
}

pub fn _string(state: StringParseState, source: &[u8], position: usize) -> Result<Context> {
    match state {
        StringParseState::Starting => {
            if let Some(byte) = source.get(position) {
                if byte == &b'"' {
                    _string(StringParseState::Consuming, source, position + 1)
                } else {
                    let result = Error::Generic(f!("expected a '\"', got '{}'", byte.as_char()));

                    Err(result)
                }
            } else {
                let result = Error::Generic("expected a '\"', got none".to_string());

                Err(result)
            }
        }

        StringParseState::Consuming => {
            if let Some(byte) = source.get(position) {
                if byte == &b'"' {
                    _string(StringParseState::Ending, source, position)
                } else if byte == &b'\\' {
                    let position = position + 1;

                    if let Some(byte) = source.get(position) {
                        if byte == &b'"'
                            || byte == &b'\\'
                            || byte == &b'/'
                            || byte == &b'b'
                            || byte == &b'f'
                            || byte == &b'n'
                            || byte == &b'r'
                            || byte == &b't'
                        {
                            _string(StringParseState::Consuming, source, position + 1)
                        } else if byte == &b'u' {
                            let position = position + 1;

                            let result = resolve_hex_digits(source, position);

                            match result {
                                Ok((source, position)) => {
                                    _string(StringParseState::Consuming, source, position)
                                }
                                Err(result) => Err(result),
                            }
                        } else {
                            let result = Error::Generic(f!(
                                "expected a valid escape character, got '{}'",
                                byte.as_char()
                            ));

                            Err(result)
                        }
                    } else {
                        let result = Error::Generic(
                            "expected a valid escape character, got none".to_string(),
                        );

                        Err(result)
                    }
                } else {
                    todo!()
                }
            } else {
                let result =
                    Error::Generic("expected a '\"' or any character, got none".to_string());

                Err(result)
            }
        }

        StringParseState::Ending => todo!(),
    }
}

#[tailcall]
fn consume_hex_digits_for(length: usize, source: &[u8], position: usize) -> Result<(&[u8], usize)> {
    if length != 0 {
        if let Some(byte) = source.get(position) {
            if byte.is_ascii_hexdigit() {
                let length = length - 1;
                let position = position + 1;

                consume_hex_digits_for(length, source, position)
            } else {
                let result = Error::Generic(f!("expected a hex digit, got '{}'", byte.as_char()));

                Err(result)
            }
        } else {
            let result = Error::Generic("expected a hex digit, got none".to_string());

            Err(result)
        }
    } else {
        let result = (source, position);

        Ok(result)
    }
}

fn resolve_hex_digits(source: &[u8], position: usize) -> Result<(&[u8], usize)> {
    consume_hex_digits_for(4, source, position)
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
