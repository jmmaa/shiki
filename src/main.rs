use shiki::parser::context::Context;
use shiki::parser::primitives::number::{digits, float, integer, number};

use shiki::prelude::*;

fn main() -> Result<()> {
    let start = std::time::Instant::now();

    let steps = 100000000;

    for _ in 0..steps {
        number(Context::new(b"+123456789e123", 0)).unwrap();
    }

    let end = start.elapsed();

    let average = end / steps;

    println!("{:.2?} , {:.5?}", end, average);

    // println!("{:?}", number(Context::new(b"123e+33_3", 0)));

    Ok(())
}
