// Define a type alias for readability.
// In Rust, [u8; 32] is a fixed-size array allocated on the Stack.
pub type Hash = [u8; 32];

#[derive(Debug)]
pub struct MerklePath {
    // TODO: Define a public field named 'hashes' which is a Vector of Hash
    pub hashes: Vec<Hash>, // list of sibling hashes
}

impl MerklePath {
    /// Creates a new empty proof
    pub fn new() -> Self {
        // TODO: Return a MerklePath instance with an empty vector
        MerklePath{hashes: Vec::new(),}
    }

    /// Adds a sibling to the end of the proof
    /// Note the `&mut self`: why do we need mutability here?
    pub fn add_sibling(&mut self, hash: Hash) {
        // TODO: Push the hash into the vector
        self.hashes.push(hash);
    }

    /// Removes the last added element and returns it (LIFO)
    /// Hint: Check the .pop() method in the Vec docs
    /// Note: We return Option<Hash> because the vector might be empty
    pub fn pop_sibling(&mut self) -> Option<Hash> {
        self.hashes.pop()
    }

    /// Returns the number of elements in the proof
    /// Note the `&self`: here we borrow in read-only mode
    pub fn len(&self) -> usize {
        self.hashes.len()
    }
}

// --- Test Zone (Do not modify, but read to understand) ---
#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a dummy hash filled with `v`
    fn mock_hash(v: u8) -> Hash {
        [v; 32]
    }

    #[test]
    fn test_lifecycle() {
        // 1. Creation
        let mut path = MerklePath::new();
        assert_eq!(path.len(), 0);

        // 2. Addition (Mutation)
        path.add_sibling(mock_hash(1));
        path.add_sibling(mock_hash(2));
        assert_eq!(path.len(), 2);

        // 3. Removal (Mutation + Option)
        let last = path.pop_sibling();
        
        // Verify we retrieved Something (Some) and it is hash 2
        match last {
            Some(h) => assert_eq!(h, mock_hash(2)),
            None => panic!("Should contain a value!"),
        }
        
        assert_eq!(path.len(), 1);
    }
}