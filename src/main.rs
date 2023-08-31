#![allow(unused)]

use shiki::parser::primitives::inline_string::inline_string;
use shiki::prelude::*;

use shiki::parser::context::Context;
use shiki::parser::primitives::identifier::alphanumerics;
use shiki::parser::primitives::number::number;

fn main() -> Result<()> {
    let result = inline_string(Context::new(r#""\uffE0helloworld!""#.as_bytes(), 0));

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
