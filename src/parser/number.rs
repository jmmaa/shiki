use crate::prelude::*;

use crate::parser::primitive::number;
use crate::parser::utils::bytes_to_str;

use serde_json::Number;

pub fn create_number_node(source: &'static [u8]) -> Result<(Number, &'static [u8])> {
    let parse_result =
        number(source, 0).map(|(source, read_position)| source.split_at(read_position));

    match parse_result {
        Ok((parsed, remaining)) => {
            let num_str = bytes_to_str(parsed);

            match num_str.parse::<Number>() {
                Ok(number) => {
                    let result = (number, remaining);

                    Ok(result)
                }
                Err(error) => {
                    let result = Error::Generic(f!(
                        "cannot convert '{}' to a number, caused by {}",
                        num_str,
                        error
                    ));

                    Err(result)
                }
            }
        }

        Err(result) => Err(result),
    }
}
