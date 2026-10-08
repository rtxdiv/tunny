use crate::storage::{Storage};


pub struct TestStorage {
    text: String
}


impl TestStorage {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Storage for TestStorage {
    fn test(&self) {
        println!("{}", self.text)
    }
}