
/// A raw dynamic array.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawDynArray<'a, T> {
    buf: &'a [T],
    len: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for RawDynArray<'_,T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RawDynArray {{ buf: {:?}, len: {} }}", self.buf, self.len)
    }
}
