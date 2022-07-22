use std::fmt;
use std::any::Any;
use std::error::Error;



#[macro_export]
macro_rules! cap_panic {
    (
        $method_name: expr,
        $index: expr,
        $len: expr
    ) => {
        panic!(
            concat!(
                "CVec::",
                $method_name,
                ": index {} is out of bounds in vector of length {}"
            ),
            $index,
            $len
        )
    }
}



#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct CapacityError<T = ()> {
    element: T,
}
impl<T> CapacityError<T> {
    pub const fn new(element: T) -> CapacityError<T> {
        CapacityError {
            element: element,
        }
    }
    pub fn element(self) -> T {
        self.element
    }
    pub fn simplify(self) -> CapacityError {
        CapacityError { element: () }
    }
}
impl<T: Any> Error for CapacityError<T> {}
impl<T> fmt::Display for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "insufficient capacity")
    }
}
impl<T> fmt::Debug for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CapacityError: insufficient capacity")
    }
}