#![allow(unused)]

use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;
use shiki::parser::primitives::number::{exponent, number};

fn main() -> Result<()> {
    let result = exponent(b"123e-123.311", 0);
    println!("{:?}", String::from_utf8(result.unwrap().0.to_vec()));
    Ok(())
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
