//

pub trait ByteSliceUtil<'a> {
    fn slice_until(&self, position: usize) -> &'a [u8];
}

impl<'a> ByteSliceUtil<'a> for &'a [u8] {
    #[inline]
    fn slice_until(&self, position: usize) -> &'a [u8] {
        &self[..position]
    }
}

pub trait ByteUtil {
    fn is_ascii_nonzero_digit(&self) -> bool;

    fn is_ascii_zero_digit(&self) -> bool;

    fn is_ascii_underscore(&self) -> bool;

    fn is_ascii_plus(&self) -> bool;

    fn is_ascii_minus(&self) -> bool;

    fn is_ascii_period(&self) -> bool;

    //

    fn as_char(&self) -> char;
}

impl ByteUtil for u8 {
    #[inline]
    fn is_ascii_nonzero_digit(&self) -> bool {
        self.is_ascii_digit() && *self != b'0'
    }

    #[inline]
    fn is_ascii_zero_digit(&self) -> bool {
        *self == b'0'
    }

    #[inline]
    fn is_ascii_underscore(&self) -> bool {
        *self == b'_'
    }

    #[inline]
    fn is_ascii_plus(&self) -> bool {
        *self == b'+'
    }

    #[inline]
    fn is_ascii_minus(&self) -> bool {
        *self == b'-'
    }

    #[inline]
    fn is_ascii_period(&self) -> bool {
        *self == b'.'
    }

    #[inline]
    fn as_char(&self) -> char {
        *self as char
    }
}
