use std::fmt::{Debug, Display};
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Undefined;
impl Debug for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"undefined")
    }
}
impl Display for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Undefined")
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Null;
impl Debug for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"null")
    }
}
impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Null")
    }
}