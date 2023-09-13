use crate::prelude::*;

use tailcall::tailcall;

#[tailcall]
fn successful<'a, T>(
    input: &'a [u8],
    choices: &'a [fn(&'a [u8]) -> Result<(T, &'a [u8])>],
) -> Result<(T, &'a [u8])> {
    if let Some(parser) = choices.get(0) {
        match parser(input) {
            Ok(result) => Ok(result),
            Err(_) => successful(input, &choices[1..]),
        }
    } else {
        let result = Error::Generic("no choices left".to_string());
        Err(result)
    }
}

pub fn choice<'a, T>(
    choices: &'a [fn(&'a [u8]) -> Result<(T, &'a [u8])>],
) -> impl Fn(&'a [u8]) -> Result<(T, &'a [u8])> {
    |input: &[u8]| successful(input, choices)
}
