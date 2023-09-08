#![allow(unused)]

use shiki::parser::primitives::number::number;
use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;

use shiki::parser::primitives::keyword::kw_false;

fn main() -> Result<()> {
    // calculate_time(|| {
    //     for _ in 0..100_000_000 {
    //         number(b"1230213.012312e213", 0).unwrap();
    //     }
    // });

    // let result = number(b"1230213.012312e+213", 0);

    let result = kw_false(b"false", 0);

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
