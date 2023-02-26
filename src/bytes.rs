//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//! Byte Utilities

/// An Object of bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteObject<'a,T> {
    length: usize,
    bytes: &'a [u8],
    addr: *const T
}

#[allow(dead_code)]
impl<'a,T> ByteObject<'a,T> {
    /// Creates a new `ByteObject` from a slice of bytes and a length
    #[inline]
    #[must_use]
    pub const fn from_raw_parts(length: usize, bytes: &'a [u8], addr: *const T) -> Self {
        assert!(length == bytes.len(), "length and bytes length must be equal");
        Self { length, bytes, addr }
    }

    /// returns the length of the `ByteObject`
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    /// returns the length of the `ByteObject`
    #[inline]
    #[must_use]
    pub const fn length(&self) -> usize {
        self.len()
    }

    /// returns the length of the `ByteObject`
    #[inline]
    #[must_use]
    pub const fn ilength(&self) -> isize {
        self.length() as isize
    }

    /// returns the length of the `ByteObject`
    #[inline]
    #[must_use]
    pub const fn ilen(&self) -> isize {
        self.ilength()
    }

    /// returns if the `ByteObject` is empty
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// returns if the `ByteObject` is not empty
    #[inline]
    #[must_use]
    pub const fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// returns the bytes of the `ByteObject`
    #[inline]
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// returns a clone of the bytes of the `ByteObject`
    /// ## Safety
    /// This function is unsafe because it can cause a memory leak
    /// if the returned value is not freed.
    /// ## UNSTABLE
    /// [See here](https://doc.rust-lang.org/nightly/core/ptr/fn.read.html)
    #[inline]
    #[must_use]
    pub const unsafe fn as_object(&self) -> T {
        let res: T = core::ptr::read(self.addr);
        res 
    }
}

impl<'a,T> From<&T> for ByteObject<'a,T> {
    fn from(t: &T) -> Self {
        let addr: *const T = &*t;
        let bytes: &'a [u8] = unsafe {
            core::slice::from_raw_parts(addr as *const u8, core::mem::size_of::<T>())
        };
        Self::from_raw_parts(bytes.len(), bytes, addr)
    }
}

/// Allows the cloning of an object that does not implement `Clone`
/// ## Safety
/// This function is unsafe because it can cause a memory leak
/// if the returned value is not freed.
/// ## Example
/// ```rust
/// use memutils::*;
/// 
/// struct UnCloneable {
///     data: u8
/// }
/// 
/// fn main() {
///     let original = UnCloneable { data: 10 };    
///     let new = clone!(&original);
///     assert!(new.data == original.data)
/// }
/// ```
#[macro_export]
macro_rules! clone {
    ($i:expr) => {
        unsafe {
            unsafe fn __unsafe_clone<T>(t: &T) -> T {
                let original = crate::ByteObject::from(t);
                assert!(original.len() > 0);
                let res: T = original.as_object();
                return res;
            }
            __unsafe_clone($i)
        }
    };
    () => ();
}

#[test]
fn byteobject_test() {
    struct UnCloneable {
        data: u8
    }
    let original = UnCloneable { data: 10 };    
    let new = clone!(&original);
    assert!(new.data == original.data)
}