use crate::prelude::*;

use tailcall::tailcall;

use crate::parser::primitive::alphanumeric;
/// This parser is represented with the bnf grammar below
/// ```bnf
/// <alphanumerics> ::= <alphanumeric> | <alphanumeric> <alphanumerics>
/// ```
#[tailcall]
pub fn alphanumerics(source: &'static [u8], read_position: usize) -> Result<Context> {
    match alphanumeric(source, read_position) {
        Ok(result) => {
            let (source, read_position) = result;

            match alphanumeric(source, read_position) {
                Ok(_) => alphanumerics(source, read_position),
                Err(_) => {
                    if let Some(b'_') = source.get(read_position) {
                        alphanumerics(source, read_position + 1)
                    } else {
                        let result = (source, read_position);

                        Ok(result)
                    }
                }
            }
        }
        Err(result) => Err(result),
    }
}

#[cfg(test)]
mod test_alphanumerics {

    use crate::parser::primitive::alphanumerics;

    #[test]
    fn test_alphanumerics_parser() {
        assert!(alphanumerics(b"0", 0).is_ok());
        assert!(alphanumerics(b"1", 0).is_ok());
        assert!(alphanumerics(b"a", 0).is_ok());
        assert!(alphanumerics(b"A", 0).is_ok());
        assert!(alphanumerics(b"&", 0).is_err());
        assert!(alphanumerics(b"", 0).is_err());

        let case1 = alphanumerics(b"0123asd", 0).unwrap();

        assert_eq!(case1.1, 7);
    }
}
