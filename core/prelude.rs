//! Utilities for raw pointers

/// A trait for raw pointers.
pub trait PointerUtils<T> {
    /// Returns a mutable pointer to the object.
    fn mp(&mut self) -> *mut T;
    /// Returns a const pointer to the object.
    fn cp(&self) -> *const T;
}

/// A trait for objects that can be created from a raw pointer.
pub trait FromRawPointer<T> {
    /// Creates an object from a raw pointer.
    unsafe fn from_raw_pointer(ptr: *mut T) -> T;
}

/// A trait that allows objects to be cloned by copying bytes.
pub trait ByteClone {
    /// Clones the object by copying bytes.
    unsafe fn byte_clone(&self) -> Self;
}

impl<T> FromRawPointer<T> for T {
    unsafe fn from_raw_pointer(ptr: *mut T) -> T {
        let original = crate::ByteObject::from(ptr);
        assert!(original.len() > 0);
        let res: T = original.as_object();
        return res;
    }
}

impl<T> PointerUtils<T> for T {
    fn mp(&mut self) -> *mut T {
        return self as *mut T;
    }
    fn cp(&self) -> *const T {
        return self as *const T;
    }
}

impl<T> ByteClone for T {
    unsafe fn byte_clone(&self) -> Self {
        let original = crate::ByteObject::from(self);
        assert!(original.len() > 0);
        let res = original.as_object();
        res
    }
}
