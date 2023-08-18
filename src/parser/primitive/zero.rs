use crate::prelude::*;

/// This parser is represented with the bnf grammar below
/// ```bnf
/// <zero> ::= "0"
/// ```
pub fn zero(source: &'static [u8], read_position: usize) -> Result<(&'static [u8], usize)> {
    match source.get(read_position) {
        Some(byte) => {
            if *byte == b'0' {
                let result = (source, read_position + 1);

                Ok(result)
            } else {
                let result = Error::Generic(f!("expected a zero, got '{}'", *byte as char));

                Err(result)
            }
        }

        None => {
            let result = Error::Generic("expected a zero, got none".to_string());

            Err(result)
        }
    }
}

#[cfg(test)]
mod test_zero {
    use crate::parser::primitive::zero::zero;

    #[test]
    fn test_zero_parser() {
        assert!(zero(b"0", 0).is_ok());
        assert!(zero(b"1", 0).is_err());

        let case1 = zero(b"0", 0).unwrap();

        assert_eq!(case1.1, 1);
    }
}
