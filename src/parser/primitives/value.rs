use crate::prelude::*;

use crate::parser::primitives::identifier::identifier;
use crate::parser::primitives::keyword::kw_false;
use crate::parser::primitives::keyword::kw_null;
use crate::parser::primitives::keyword::kw_true;
use crate::parser::primitives::number::number;
use crate::parser::primitives::string::string;

pub fn value(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    string(src, pos)
        .or(identifier(src, pos))
        .or(kw_false(src, pos))
        .or(kw_null(src, pos))
        .or(kw_true(src, pos))
        .or(number(src, pos))
}
