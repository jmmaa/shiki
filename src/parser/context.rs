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
    pub fn get_current_byte(&self) -> Option<&u8> {
        self.s.get(self.p)
    }

    #[inline]
    pub fn get_current_slice(&self) -> &'src [u8] {
        &self.s[..self.p]
    }

    /// this consumes the current context and creates a new
    /// one that has an incremented position
    // #[inline]
    // pub fn get_next_context(self) -> Context<'src> {
    //     Context::new(self.source(), self.position() + 1)
    // }

    #[inline]
    pub fn new(source: &'src [u8], position: usize) -> Context<'src> {
        Context {
            s: source,
            p: position,
        }
    }
}
