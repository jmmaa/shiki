#![allow(unused)]

use shiki::prelude::*;

use shiki::parser::context::Context;
use shiki::parser::primitives::identifier::alphanumerics;
use shiki::parser::primitives::number::number;

fn main() -> Result<()> {
    // let start = std::time::Instant::now();

    // let steps = 100_000_000;

    // for _ in 0..steps {
    //     number(Context::new(b"+12345.6789e123", 0)).unwrap();
    // }

    // for _ in 0..steps {
    //     alphanumerics_split_position(b"a123davasd123a42", 0).unwrap();
    // }

    // let end = start.elapsed();

    // let average = end / steps;

    // println!("{:.2?} , {:.5?}", end, average);

    let source = b"+12345.6789e123";

    calculate_time(|| {
        // let sample = &[1u8, 2u8, 3u8, 4u8, 5u8];

        // let sub = &sample[1..sample.len() - 1];
        let steps = 100_000_000;

        for _ in 0..steps {
            number(Context::new(b"+12345.6789e123", 0)).unwrap();
            // alphanumerics(Context::new(b"a123davasd123a42", 0)).unwrap();
        }
    });

    // let num = number(Context::new(source, 0)).unwrap();

    // println!("{num:?}");

    Ok(())
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
