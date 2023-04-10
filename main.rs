#![deny(unsafe_code)]

#[allow(unused)]mod privobj;#[allow(unused)]use privobj::*;

use memutils::*;

#[not_safe]
fn main() {

    let obj = PrivateObject::new("Hello World!");
    println!("{:?}", &obj);

    // obj.private = "Hello New World!";

    let mut new_obj = mem!(
        obj as {
            pub(crate) private: &'static str,
        }
    );

    new_obj.private = "Hello New World!";

    let obj = mem!(
        turn new_obj into PrivateObject<&str>
    );

    println!("{:?}", obj)

}
