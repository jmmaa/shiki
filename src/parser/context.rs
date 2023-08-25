#[derive(Debug)]
pub struct Context {
    s: &'static [u8],
    p: usize,
}

impl Context {
    #[inline]
    pub fn source(&self) -> &'static [u8] {
        self.s
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.p
    }

    #[inline]
    pub fn new(source: &'static [u8], position: usize) -> Context {
        Context {
            s: source,
            p: position,
        }
    }
}
