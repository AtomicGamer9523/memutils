#![deny(unsafe_code)]

#[test]
fn no_heap_corrupt_test() {
    use memutils::*;

    struct Object {
        data: u32,
    }
    
    let mut c = Object { data: 0 };
    c.data = 10;

    let c2 = clone!(&c);

    assert!(c2.data == 10)

}
