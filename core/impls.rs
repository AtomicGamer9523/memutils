
use core::marker;
use core::slice;
use core::ptr;
use core::mem;
use core::fmt;

use crate::*;

impl<'a,T> ByteObject<'a,T> {
    /// Creates a new `ByteObject` from a slice of bytes and a length
    #[inline]
    #[must_use]
    pub const fn from_raw_parts(length: usize, bytes: &'a [u8], addr: *const T) -> Self {
        assert!(length == bytes.len(), "length and bytes length must be equal");
        Self { length, bytes, addr }
    }

    /// Creates a new `ByteObject` from a slice of bytes and a length
    #[inline]
    #[must_use]
    pub const fn addr(&self) -> *const T {
        self.addr
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
    pub unsafe fn as_object(&self) -> T {
        let res = create_object_byteguard_from_pointer(self.addr);
        res.into_inner()
    }
}

impl<'a,T> From<&T> for ByteObject<'a,T> {
    fn from(t: &T) -> Self {
        let addr: *const T = &*t;
        let bytes: &'a [u8] = unsafe {
            slice::from_raw_parts(addr as *const u8, mem::size_of::<T>())
        };
        Self::from_raw_parts(bytes.len(), bytes, addr)
    }
}

impl<'a,T> From<&mut T> for ByteObject<'a,T> {
    fn from(t: &mut T) -> Self {
        ByteObject::from(&*t)
    }
}

impl<'a,T> From<*mut T> for ByteObject<'a,T> {
    fn from(t: *mut T) -> Self {
        unsafe {
            ByteObject::from(&*t)
        }
    }
}

impl<'a,T> From<*const T> for ByteObject<'a,T> {
    fn from(t: *const T) -> Self {
        unsafe {
            ByteObject::from(&*t)
        }
    }
}

impl<T> fmt::Debug for ByteGuard<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ByteGuard")
            .field("ptr", &self.ptr)
            .field("layout", &self.layout)
            .finish()
    }
}

impl<T> ByteGuard<T> {
    /// Creates a new `ByteGuard` from a pointer to a type.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub(crate) fn new(ptr: *mut u8, layout: crate::Layout) -> Self {
        if ptr.is_null() {
            crate::handle_alloc_error(layout);
        }
        Self { ptr, layout, _marker: marker::PhantomData }
    }

    /// Returns a pointer to the guarded value.
    #[inline]
    #[must_use = "if you don't use the result, the value will be dropped immediately"]
    pub fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    /// Returns a mutable pointer to the guarded value.
    #[inline]
    #[must_use = "if you don't use the result, the value will be dropped immediately"]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr as *mut T
    }

    /// Returns the guarded value.
    /// Consumes the guard.
    #[inline]
    #[must_use = "if you don't use the result, the value will be dropped immediately"]
    pub unsafe fn into_inner(self) -> T {
        let res: T = ptr::read(self.as_ptr());
        drop(self);
        res
    }

    /// Copies the value from the given pointer to the guarded value.
    /// ## Safety
    /// This function is unsafe because it can cause a memory leak
    /// if the returned value is not freed.
    #[inline]
    pub(crate) unsafe fn copy_from(&mut self, src: *const T) {
        ptr::copy_nonoverlapping(src, self.as_mut_ptr(), 1);
    }
}

impl<T> AsRef<T> for ByteGuard<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }
}

impl<T> AsMut<T> for ByteGuard<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

impl<T> Drop for ByteGuard<T> {
    fn drop(&mut self) {
        unsafe { crate::dealloc(self.ptr, self.layout) }
    }
}
