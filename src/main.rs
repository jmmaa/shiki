#![allow(unused)]

use shiki::parser::primitives::number::number;
use shiki::prelude::*;

use shiki::parser::primitives::identifier::alphanumerics;

use shiki::parser::primitives::keyword::kw_false;

use shiki::parser::combinator::choice::choice;

fn main() -> Result<()> {
    let result = kw_false(b"false", 0);

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
