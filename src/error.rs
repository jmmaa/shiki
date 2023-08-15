#[derive(Debug, PartialEq, Eq)]

pub struct Error {
    i: usize,
}

impl Error {
    pub fn new(i: usize) -> Error {
        Error { i }
    }
    pub fn index(&self) -> usize {
        self.i
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
