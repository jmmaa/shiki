#![allow(unused)]

use shiki::parser::primitives::number::number;
use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;

use shiki::parser::primitives::keyword::kw_false;

// use shiki::parser::primitive::*;

// fn main() -> Result<()> {
//     let result = kw_false(b"false", 0);

//     println!("{:?}", result);

//     println!("{}", String::from_utf8(result.unwrap().0.to_vec()).unwrap());

//     Ok(())
// }

fn digit(input: &[u8]) -> Result<(&u8, &[u8])> {
    if let Some(byte) = input.first() {
        if byte.is_ascii_digit() {
            Ok((byte, &input[1..]))
        } else {
            let result = Error::Generic(f!("expected digit, got '{}'", *byte as char));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected digit, got none".to_string());

        Err(result)
    }
}

fn alpha(input: &[u8]) -> Result<(&u8, &[u8])> {
    if let Some(byte) = input.first() {
        if byte.is_ascii_alphabetic() {
            Ok((byte, &input[1..]))
        } else {
            let result = Error::Generic(f!("expected alpha, got '{}'", *byte as char));

            Err(result)
        }
    } else {
        let result = Error::Generic("expected alpha, got none".to_string());

        Err(result)
    }
}

fn main() -> Result<()> {
    Ok(())
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
