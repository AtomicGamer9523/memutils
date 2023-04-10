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

//! Includes safe wrappers for `Null` and `Undefined` values.

use core::fmt::{
    Display,
    Debug,
    self
};

/// A safe wrapper for Undefined Behavior.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Undefined;
impl Debug for Undefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"undefined")
    }
}
impl Display for Undefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Undefined")
    }
}

/// A safe wrapper for `ptr::null()`.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Null<T: ?Sized + core::ptr::Thin> {
    _private: *const T
}
impl<T: ?Sized + core::ptr::Thin> Null<T> {
    
    /// Creates a new `Null` instance.
    #[inline(always)]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _private: core::ptr::null()
        }
    }
}
impl<T> From<T> for Null<T> {
    /// Ignores the value and returns a `Null` instance.
    fn from(_: T) -> Self {
        Self::new()
    }
}
impl<T: ?Sized + core::ptr::Thin> Debug for Null<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"null")
    }
}
impl<T: ?Sized + core::ptr::Thin> Display for Null<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Null")
    }
}
