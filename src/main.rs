#![allow(unused)]

use shiki::parser::primitives::number::number;
use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;
fn main() -> Result<()> {
    // calculate_time(|| {
    //     for _ in 0..100_000_000 {
    //         number(b"1230213.012312e213", 0).unwrap();
    //     }
    // });

    let result = number(b"1230213.012312e+213", 0);

    // ADD MORE TESTS TO THIS!
    println!("{:?}", result);

    println!("{}", String::from_utf8(result.unwrap().0.to_vec()).unwrap());

    Ok(())

    // improve number later
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
