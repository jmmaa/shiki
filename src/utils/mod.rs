pub trait ByteUtil {
    fn is(&self, c: char) -> bool;

    fn is_not(&self, c: char) -> bool;

    fn as_char(&self) -> char;
}

impl ByteUtil for u8 {
    // maybe add an enum for the is_not/is methods

    #[inline]
    fn is(&self, c: char) -> bool {
        c == (*self as char)
    }
    #[inline]
    fn is_not(&self, c: char) -> bool {
        c != (*self as char)
    }

    #[inline]
    fn as_char(&self) -> char {
        *self as char
    }
}
