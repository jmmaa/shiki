use crate::prelude::*;

use crate::utils::ByteUtil;

pub fn tag<'src>(
    target: &[u8],
    src: Source<'src>,
    pos: Position,
) -> Result<(&'src [u8], Source<'src>, Position)> {
    if let Some(target_byte) = target.first() {
        if let Some(byte) = src.get(pos) {
            if byte == target_byte {
                tag(&target[1..], src, pos + 1)
            } else {
                let result = Error::Generic(f!(
                    "expected a '{}', got '{}'",
                    target_byte.as_char(),
                    byte.as_char()
                ));

                Err(result)
            }
        } else {
            let result = Error::Generic(f!("expected a '{}', got none", target_byte.as_char()));

            Err(result)
        }
    } else {
        let parsed = &src[..pos];
        let result = (parsed, src, pos);

        Ok(result)
    }
}

pub fn kw_true(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    tag(b"true", src, pos)
}

pub fn kw_false(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    tag(b"false", src, pos)
}

pub fn kw_null(src: Source, pos: Position) -> Result<(&[u8], Source, Position)> {
    tag(b"null", src, pos)
}
