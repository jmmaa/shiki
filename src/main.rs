#![allow(unused)]

use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;
use shiki::parser::primitives::number::{exponent, number};

fn main() -> Result<()> {
    calculate_time(|| {
        for _ in 0..100_000_000 {
            number(b"1230213012312", 0).unwrap();
        }
    });

    let result = number(b"1230213012312", 0);

    // ADD MORE TESTS TO THIS!
    println!("{:?}", result);

    println!("{}", String::from_utf8(result.unwrap().0.to_vec()).unwrap());

    Ok(())
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
