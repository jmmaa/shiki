use crate::prelude::*;

use super::{alphanumeric, alphanumerics, letter};

pub fn identifier(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match letter(source, read_position) {
        Ok(result) => {
            let (source, read_position) = result;

            match alphanumeric(source, read_position) {
                Ok(_) => alphanumerics(source, read_position),
                Err(_) => {
                    let result = (source, read_position);

                    Ok(result)
                }
            }
        }
        Err(result) => Err(result),
    }
}
