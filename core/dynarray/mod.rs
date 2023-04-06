//! Dynamic array.

mod raw;
#[cfg(feature = "reveal_hidden")]
pub use raw::RawDynArray;
#[cfg(not(feature = "reveal_hidden"))]
pub(crate) use raw::RawDynArray;

/// A dynamic array.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynArray<'a, T> {
    buf: RawDynArray<'a, T>,
    len: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for DynArray<'_,T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DynArray {{ buf: {:?} }}", self.buf)
    }
}
