#![deny(unsafe_code)]

#[allow(unused)]
use memutils::*;

#[test]
#[not_safe]
fn no_heap_corrupt_test() {
    #[repr(transparent)]
    struct Object {
        data: u32,
    }
    
    let mut c = Object { data: 0 };
    c.data = 10;

    let c2 = clone!(&c);

    assert!(c2.data == 10)

}
