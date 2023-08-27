#[derive(Debug)]
pub struct Context<'src> {
    s: &'src [u8],
    p: usize,
}

impl<'src> Context<'src> {
    #[inline]
    pub fn source(&self) -> &'src [u8] {
        self.s
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.p
    }

    #[inline]
    pub fn get_byte(&self) -> Option<&u8> {
        self.s.get(self.p)
    }

    #[inline]
    pub fn get_left_slice(&self) -> &'src [u8] {
        &self.s[..self.p]
    }

    #[inline]
    pub fn new(source: &'src [u8], position: usize) -> Context<'src> {
        Context {
            s: source,
            p: position,
        }
    }
}
