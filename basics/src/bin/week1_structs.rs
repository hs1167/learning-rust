// DO THIS EXERCISE
// File: src/week1_ownership.rs

pub struct Secret {
    bytes: Vec<u8>,
}

impl Secret {
    pub fn new(bytes: Vec<u8>) -> Self {
        Secret { bytes }
    }
    
    // This doesn't compile! Why? (Ownership moved)
    pub fn leak(&self) -> Vec<u8> {
        self.bytes.clone() // ❌ If we remove .clone(), why does it fail?
    }
}

fn main() {
    let secret = Secret::new(vec![1, 2, 3]);
    let s1 = secret; // ownership moved
    // let s2 = secret; // ❌ ERROR: value used after move
}