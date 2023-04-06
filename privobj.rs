#![allow(dead_code, unused)]

type PrivateObjectData = &'static str;

#[derive(Debug)]
pub struct PrivateObject {
    private: PrivateObjectData,
}

impl PrivateObject {
    pub fn new() -> Self {
        Self { private: "private" }
    }

    pub fn get_data(&self) -> PrivateObjectData {
        self.private
    }
}