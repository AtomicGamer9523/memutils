#![allow(dead_code, unused)]

#[repr(transparent)]
#[derive(Debug)]
pub struct PrivateObject<T> {
    private: T,
}

impl<T> PrivateObject<T> {
    pub fn new(v: T) -> Self {
        Self { private: v }
    }

    pub fn get_data(&self) -> &T {
        &self.private
    }
}