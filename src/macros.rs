/// Allows the cloning of an object that does not implement `Clone`
/// ## Safety
/// This function is unsafe because it can cause a memory leak
/// if the returned value is not freed.
/// ## Example
/// ```rust
/// use memutils::*;
/// 
/// struct Class {
///     data: u32,
/// }
/// 
/// fn main() {
///     let mut c = Class { data: 0 };
///     c.data = 10;
/// 
///     let c2 = clone!(&c);
///     
///     println!("{:?}", c2)
/// }
/// ```
#[macro_export]
macro_rules! clone {
    ($i:expr) => {
        {
            use memutils::prelude::*;
            ($i).byte_clone()
        }
    };
    () => ();
}
