// File: src/week1_ownership.rs

pub struct Secret {
    bytes: Vec<u8>,
}

impl Secret {
    pub fn new(bytes: Vec<u8>) -> Self {
        Secret { bytes }
    }

    // Takes ownership, consumes the Secret, returns the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

fn main() {
    let secret = Secret::new(vec![1, 2, 3]);
    
    // MOVE semantic: ownership transfers from 'secret' to 's1'.
    let _s1 = secret; 
    
    // COMPILE ERROR: Use of moved value: 'secret'.
    // The value layout in memory is now owned by 's1'.
    // let s2 = secret; 
}