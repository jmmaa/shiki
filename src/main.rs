#![allow(unused)]

use shiki::prelude::*;

use shiki::parser::primitives::keyword::kw_true;

use shiki::parser::primitives::string::string;

use shiki::parser::primitives::string::multiline_string_chars;
use shiki::parser::primitives::value::value;

fn main() -> Result<()> {
    println!(
        "{:?}",
        value(
            br#""""
    ggwp
    """"#,
            0
        )
    );

    Ok(())
}

fn calculate_time(op: impl Fn()) {
    let start = std::time::Instant::now();

    op();

    let end = start.elapsed();

    println!("{:.2?}", end);
}
