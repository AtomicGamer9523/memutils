//! This crate contains the `#[memutils::nonsafe]` macro.

#![no_std]

#![forbid(
    missing_debug_implementations,
    future_incompatible,
    missing_docs,
)]
#![warn(
    deprecated_in_future,
    clippy::all,
    dead_code,
    warnings,
    unused
)]

#[doc(hidden)]
#[allow(unused_extern_crates)]
extern crate proc_macro;
#[doc(hidden)]
use proc_macro::TokenStream;

/// Allows the creation of an unsafe function that is not marked as unsafe.
/// Bypasses the `unsafe_code` lint.
/// 
/// ## Example
/// ```rust
/// #![deny(unsafe_code)]
/// use memutils::*;
/// 
/// #[derive(Debug)]
/// struct Class {
///     data: u32,
/// }
/// 
/// #[not_safe]
/// fn unsafefunc(ptr: *mut Class) {
///     (*ptr).data = 1;
/// }
/// 
/// fn main() {
///     let mut c = Class { data: 0 };
///     let ptr = c.mp();// returns a mutable pointer to the object
/// 
///     drop(c);
/// 
///     unsafefunc(ptr);
/// 
///     let c2 = ptr;
///     println!("{:?}", c2);
/// }
/// ```
#[proc_macro_attribute]
pub fn not_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    let result = quote::quote! {
        #(#attrs)*
        #[allow(unsafe_code)]
        #[allow(unused_unsafe)]
        #vis fn #name(#inputs) #ret {
            unsafe {
                #body
            }
        }
    };

    result.into()
}
