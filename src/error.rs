// use thiserror::Error;

// use crate::prelude::{Position, Source};

// #[derive(Error, Debug)]
// pub enum Error<'src, T> {
//     #[error("{0}")]
//     Generic(String),

//     #[error("{0}")]
//     Parser((T, Source<'src>, Position)),
// }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
}
