#![deny(unsafe_code)]
use memutils::*;

mod privobj;use privobj::*;

#[not_safe]
fn main() {
    let obj = PrivateObject::new();
    println!("{:?}", &obj);

    // obj.private = "public!";

    let mut new_obj = mem!(
        obj as {
            pub private: &'static str,
        }
    );

    new_obj.private = "Hello World!";

    let obj = mem!(turn new_obj into PrivateObject);

    println!("{:?}", obj)
}
