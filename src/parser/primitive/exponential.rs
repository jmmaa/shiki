use crate::prelude::*;

use super::{digits, float, integer};

pub fn exponential(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    let result = float(source, read_position).or(integer(source, read_position));

    match result {
        Ok((source, read_position)) => match source.get(read_position) {
            Some(byte) => {
                if byte == &b'e' {
                    let read_position = read_position + 1;

                    match source.get(read_position) {
                        Some(byte) => {
                            if byte == &b'+' || byte == &b'-' {
                                let read_position = read_position + 1;

                                digits(source, read_position)
                            } else {
                                digits(source, read_position)
                            }
                        }
                        None => {
                            let result =
                                Error::Generic("expected a digit or (+, -), got none".to_string());

                            Err(result)
                        }
                    }
                } else {
                    let result = Error::Generic(f!("expected an 'e', got '{}'", *byte as char));

                    Err(result)
                }
            }

            None => {
                let result = Error::Generic("expected an 'e', got none".to_string());

                Err(result)
            }
        },

        Err(result) => Err(result),
    }
}
