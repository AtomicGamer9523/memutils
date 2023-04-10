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

use core::marker;

/// Creates a new `ByteGuard` from a pointer to a type.
#[inline]
#[must_use = "this returns the result of the operation, without modifying the original"]
pub unsafe fn create_object_byteguard_from_pointer<T>(src: *const T) -> ByteGuard<T> {
    let layout = crate::Layout::new::<T>();
    let ptr = crate::malloc(layout);
    let mut guard: ByteGuard<T> = ByteGuard::<T>::new(ptr, layout);
    guard.copy_from(src);
    guard
}

/// An Object of bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteObject<'a,T> {
    pub(crate) length: usize,
    pub(crate) bytes: &'a [u8],
    pub(crate) addr: *const T
}

/// A guard that frees the memory when dropped.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ByteGuard<T> {
    pub(crate) ptr: *mut u8,
    pub(crate) layout: crate::Layout,
    pub(crate) _marker: marker::PhantomData<T>,
}

/// ## GO
/// **G**et **O**bject from const pointer.
/// 
/// Returns a mutable reference to the object.
pub const unsafe fn go<'a, T>(ptr: *const T) -> &'a T {
    &*ptr
}

/// ## GOM
/// **G**et **O**bject **M**ut from mutable pointer.
/// 
/// Returns a mutable reference to the object.
pub const unsafe fn gom<'a, T>(ptr: *mut T) -> &'a mut T {
    &mut *ptr
}
