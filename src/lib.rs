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

//! A library with many useful utilities when dealing with memory and pointers.

#![no_std]

#![feature(ptr_metadata)]
#![feature(ptr_internals)]
#![feature(const_ptr_read)]
#![feature(rustc_attrs)]

#![forbid(
    deprecated_in_future,
    future_incompatible,
    missing_docs,
    missing_debug_implementations
)]
#![warn(
    unused,
    dead_code,
    warnings,
    clippy::all,
)]

#[cfg(feature = "reveal_hidden")]
pub extern crate alloc;
#[cfg(not(feature = "reveal_hidden"))]
extern crate alloc;
#[cfg(feature = "reveal_hidden")]
pub mod nulls;
#[cfg(not(feature = "reveal_hidden"))]
pub(crate) mod nulls;
#[cfg(feature = "reveal_hidden")]
pub mod bytes;
#[cfg(not(feature = "reveal_hidden"))]
pub(crate) mod bytes;


pub use alloc::alloc::{
    alloc as malloc,
    dealloc,
    realloc,
    Layout
};
pub use bytes::ByteObject;
pub use nulls::{Null, Undefined};
