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

#![feature(const_mut_refs)]
#![feature(ptr_metadata)]

#![forbid(
    missing_debug_implementations,
    missing_docs,
)]
#![deny(
    future_incompatible
)]
#![warn(
    deprecated_in_future,
    clippy::all,
    dead_code,
    warnings,
    unused,
)]

#[cfg(feature = "reveal_hidden")]
#[allow(pub_use_of_private_extern_crate, forbidden_lint_groups, future_incompatible, unused)]
pub extern crate alloc as liballoc;
#[cfg(not(feature = "reveal_hidden"))]
#[doc(hidden)]
extern crate alloc as liballoc;
#[cfg(feature = "reveal_hidden")]
pub mod nulls;
#[cfg(not(feature = "reveal_hidden"))]
#[doc(hidden)]
pub(crate) mod nulls;
#[cfg(feature = "reveal_hidden")]
pub mod bytes;
#[cfg(not(feature = "reveal_hidden"))]
#[doc(hidden)]
pub(crate) mod bytes;
#[cfg(feature = "reveal_hidden")]
pub mod dynarray;
#[cfg(not(feature = "reveal_hidden"))]
#[doc(hidden)]
pub(crate) mod dynarray;
#[cfg(feature = "reveal_hidden")]
pub mod prelude;
mod impls;
pub use liballoc::alloc::{
    handle_alloc_error,
    alloc as malloc,
    dealloc,
    realloc,
    Layout
};
pub use bytes::*;
pub use nulls::*;
