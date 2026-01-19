// Define a type alias for readability.
// In Rust, [u8; 32] is a fixed-size array allocated on the Stack.
pub type Hash = [u8; 32];

#[derive(Debug)]
pub struct MerklePath {
    // TODO: Define a public field named 'hashes' which is a Vector of Hash
    pub hashes: Vec<Hash>, // list of sibling hashes: this is a vertical list btw!!
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

    /// Simulates the verification path.
    /// For each hash in the proof, print whether we should concatenate
    /// the sibling on the LEFT or RIGHT based on the current index.
    ///
    /// target_index : The index of the initial leaf we want to prove.
    ///
    /// Returns : A boolean (always true for this exercise, just for structure)
    pub fn debug_verify_path(&self, mut index: usize) -> bool {
        println!("Start verification for leaf {}", index);

        // TODO: Use a `for` loop to iterate over `self.hashes`.
        // Constraint: You must iterate over REFERENCES (&Hash), do not consume the vector!
        // Hint: Use .iter().enumerate() to get both the level (depth) and the sibling.
        
        // INSIDE THE LOOP:
        // 1. Determine the sibling's position using the parity of 'index'.
        //    - If index is EVEN: The sibling is on the RIGHT.
        //    - If index is ODD: The sibling is on the LEFT.
        for (idx, sibling) in self.hashes.iter().enumerate() { //.iter() it's the same as a reference, but we must use it if we want to use enumerate() too
        // 2. Print (println!): "Level X: My index is {index}, sibling {hash:?} is on {Position}"
            if index%2 == 0 {println!("Level {idx}: My index is {index}, sibling {sibling:?} is on right!")}
                else{println!("Level {idx}: My index is {index}, sibling {sibling:?} is on left!")}
        // 3. Update 'index' for the next level (integer division by 2).
            index /= 2; //which is the same as: index=index/2
        // END OF LOOP
        }
        println!("Arrived at the (simulated) root!");
        true
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

#[cfg(test)]
mod tests_tp2 {
    use super::*;

    #[test]
    fn test_path_logic() {
        let mut path = MerklePath::new();
        // Fill with 3 dummy hashes
        path.add_sibling([10u8; 32]); // Level 0
        path.add_sibling([20u8; 32]); // Level 1
        path.add_sibling([30u8; 32]); // Level 2

        // Simulation: I am leaf 2 (binary 010)
        // Expected Logic:
        // Level 0 (idx 2, even) -> Sibling on Right. New idx -> 1
        // Level 1 (idx 1, odd)  -> Sibling on Left.  New idx -> 0
        // Level 2 (idx 0, even) -> Sibling on Right. New idx -> 0
        
        // Observe console output with: cargo test -- --nocapture
        path.debug_verify_path(2);
    }
}