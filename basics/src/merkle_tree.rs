use sha2::{Sha256, Digest};
pub type Hash = [u8;32];
/// A single node in the Merkle tree
#[derive(Clone, Debug)]
pub struct MerkleNode {
    pub hash: [u8; 32],
}

impl MerkleNode {
    /// Create a leaf node by hashing data
    pub fn leaf(data: &[u8]) -> Self {
        // TODO: Hash the data with SHA-256, store the 32-byte result
        todo!()
    }

    /// Create an internal node by hashing two children
    /// The order matters: hash(left || right), not (right || left)
    pub fn parent(left: &MerkleNode, right: &MerkleNode) -> Self {
        // TODO: Hash the concatenation of left.hash and right.hash
        todo!()
    }
}

/// A complete Merkle tree with power-of-2 leaves
pub struct MerkleTree {
    // TODO: Store the tree structure. Options:
    // Option 1: Store all layers as Vec<Vec<MerkleNode>>
    // Option 2: Store only the root and allow reconstruction
    // (I recommend Option 1 for clarity)
}

impl MerkleTree {
    /// Build a tree from a list of data items
    /// 
    /// # Panics
    /// If data.len() is not a power of 2
    pub fn new(data: Vec<&[u8]>) -> Self {
        // TODO:
        // 1. Validate that data.len() is a power of 2
        // 2. Create leaf nodes for all data items
        // 3. Build layers bottom-up until you reach a single root
        // 4. Store the structure for later queries
        todo!()
    }

    /// Return the root hash
    pub fn root(&self) -> &[u8; 32] {
        // TODO: Return a reference to the root hash
        todo!()
    }

    /// Return the depth (number of levels - 1)
    /// Example: A tree with 4 leaves has depth 2
    pub fn depth(&self) -> usize {
        // TODO: Calculate from the number of leaves
        // depth = log2(num_leaves)
        todo!()
    }

    /// Return the number of leaves
    pub fn num_leaves(&self) -> usize {
        // TODO: Return the count
        todo!()
    }

    /// Get a proof path for leaf at index `leaf_index`
    /// Returns the sibling hashes needed to reconstruct the root
    /// 
    /// # Returns
    /// A vector of (hash, direction) tuples where:
    /// - hash: the sibling hash at this level
    /// - direction: whether the sibling is on the left or right
    pub fn proof_path(&self, leaf_index: usize) -> Option<Vec<([u8; 32], ProofDirection)>> {
        // TODO: Walk from the leaf to the root, collecting siblings
        // (Optional: only implement if you have time)
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProofDirection {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tier 1: Basics
    #[test]
    #[should_panic]
    fn test_single_leaf_panics() {
        MerkleTree::new(vec![b"hello"]);
    }

    #[test]
    fn test_two_leaves() {
        let tree = MerkleTree::new(vec![b"alice", b"bob"]);
        assert_eq!(tree.num_leaves(), 2);
        assert_eq!(tree.depth(), 1);
        assert_eq!(tree.root().len(), 32);
    }

    #[test]
    fn test_four_leaves() {
        let tree = MerkleTree::new(vec![b"a", b"b", b"c", b"d"]);
        assert_eq!(tree.num_leaves(), 4);
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_leaf_count_panics() {
        MerkleTree::new(vec![b"a", b"b", b"c"]);
    }

    // Tier 2: Determinism
    #[test]
    fn test_root_deterministic() {
        let tree1 = MerkleTree::new(vec![b"alice", b"bob"]);
        let tree2 = MerkleTree::new(vec![b"alice", b"bob"]);
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_root_changes_with_data() {
        let tree1 = MerkleTree::new(vec![b"alice", b"bob"]);
        let tree2 = MerkleTree::new(vec![b"alice", b"eve"]);
        assert_ne!(tree1.root(), tree2.root());
    }
    
    // Tier 3: Scale
    #[test]
    fn test_eight_leaves() {
        let tree = MerkleTree::new(vec![
            b"leaf0", b"leaf1", b"leaf2", b"leaf3",
            b"leaf4", b"leaf5", b"leaf6", b"leaf7",
        ]);
        assert_eq!(tree.num_leaves(), 8);
        assert_eq!(tree.depth(), 3);
    }
}