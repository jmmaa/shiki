use std::ops::RangeTo;

// THIS IS JUST A TINY UTILITY FOR MAKING IT EASIER TO READ THE CODE IN GETTING SUBSLICES
pub trait Source<'a> {
    fn slice(&self, range: RangeTo<usize>) -> &'a [u8];
}

impl<'a> Source<'a> for &'a [u8] {
    fn slice(&self, range: RangeTo<usize>) -> &'a [u8] {
        &self[range]
    }
}
