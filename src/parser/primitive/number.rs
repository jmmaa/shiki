use crate::parser::primitive::float::float;
use crate::parser::primitive::integer::integer;
use crate::prelude::*;

use super::exponential;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <number> ::= <float> | <integer>
/// ```
pub fn number(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    exponential(source, read_position)
        .or(float(source, read_position).or(integer(source, read_position)))
}
