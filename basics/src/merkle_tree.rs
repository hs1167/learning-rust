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
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        MerkleNode { hash: result.into() } //into instead of try_into because of fixed 32bytes size
    }

    /// Create an internal node by hashing two children
    /// The order matters: hash(left || right), not (right || left)
    pub fn parent(left: &MerkleNode, right: &MerkleNode) -> Self {
        // TODO: Hash the concatenation of left.hash and right.hash
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);  //& to prevent useless copy
        hasher.update(&right.hash); //whithout & we make a useless copy, thats not idiomatic
        let res = hasher.finalize();
        MerkleNode { hash: res.into() }
    }
}

/// A complete Merkle tree with power-of-2 leaves
pub struct MerkleTree {
    //store all layers
    pub layers: Vec<Vec<MerkleNode>>,
}

impl MerkleTree {
    pub fn new(data: Vec<&[u8]>) -> Self {
        //Validate that data.len() is a power of 2
        if data.len() < 2 || !is_a_pow_of_two(data.len()) {panic!("The len of data must be a power of two!")} //logic negation
        //our future merkle tree
        let mut layers:Vec<Vec<MerkleNode>>= Vec::new();
        //convert data into merklenode (hash) for layer 0
        let mut convert: Vec<MerkleNode> = Vec::new();
        for elm in data {
            //layers.push(MerkleNode::leaf(elm))
            convert.push(MerkleNode::leaf(elm))
        } //data moved here
        layers.push(convert);

        //Build layers bottom-up until until reach a single root
        while layers.last().unwrap().len() > 1 { //licite her because of the push just before, so we know the vec isn't empty
            //to store the next layer
            let mut next_layer: Vec<MerkleNode>= Vec::new();
            let part = layers.last().unwrap().chunks_exact(2);
            for chunk in part{
                let new_parent  = MerkleNode::parent(&chunk[0], &chunk[1]);
                next_layer.push(new_parent)
            }
            layers.push(next_layer)
        }


        MerkleTree { layers,}
    }
    


    /// Return the root hash
    pub fn root(&self) -> Hash {
        self.layers.last().unwrap()[0].hash
    }

    ///Return the depth of the tree
    pub fn depth(&self) -> usize {
        self.layers.len() - 1
    }

    /// Return the number of leaves
    pub fn num_leaves(&self) -> usize {
        self.layers[0].len()
    }

    /// Returns
    /// A vector of (hash, direction) tuples where:
    /// - hash: the sibling hash at this level
    /// - direction: whether the sibling is on the left or right
    pub fn proof_path(&self, leaf_index: usize) -> Option<Vec<(Hash, SiblingDirection)>> {
        if leaf_index >= self.num_leaves() { return None; }
        let mut path = Vec::new();
        let mut curr_idx = leaf_index;
        for i in 0..self.depth() {
            if curr_idx%2 == 0 {path.push((self.layers[i][curr_idx+1].hash,SiblingDirection::Right))} else {
                path.push((self.layers[i][curr_idx-1].hash,SiblingDirection::Left))
            }
            curr_idx /= 2;
        }
        Some(path)
    }
}


pub fn is_a_pow_of_two (n:usize) -> bool {
        (n!=0) && (n&(n-1))==0 //because of binary rep tricks
    }

pub fn verify_proof(leaf: &[u8], proof: &[(Hash, SiblingDirection)], root: Hash) -> bool {
    let mut curr = MerkleNode::leaf(leaf).hash;
    for (sib, direction) in proof{
        let mut hasher = Sha256::new();
        match direction {
            SiblingDirection::Left => {hasher.update(sib); hasher.update(curr)}
            SiblingDirection::Right => {hasher.update(curr); hasher.update(sib)}
        }
        curr = hasher.finalize().into()
    }
    curr == root
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SiblingDirection {
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


    #[test]
fn test_sixteen_leaves() {
    let data: Vec<&[u8]> = (0..16)
        .map(|i| format!("item_{}", i))
        .map(|s| Box::leak(s.into_boxed_str()).as_bytes())
        .collect();
    
    let tree = MerkleTree::new(data);
    
    assert_eq!(tree.num_leaves(), 16);
    assert_eq!(tree.depth(), 4); // log2(16) = 4
}

    //Tier 4: Cryptographic Properties
    #[test]
fn test_avalanche_one_bit() {
    // Change one bit in one leaf => root changes completely
    let tree1 = MerkleTree::new(vec![b"hello", b"world"]);
    let tree2 = MerkleTree::new(vec![b"hallo", b"world"]); // Changed "e" to "a"
    
    let root1 = tree1.root();
    let root2 = tree2.root();
    
    // Count how many bits differ
    let mut bit_diff = 0;
    for i in 0..32 {
        let xor = root1[i] ^ root2[i];
        bit_diff += xor.count_ones() as usize;
    }
    
    // Cryptographic avalanche: should be ~50% of bits (128 out of 256)
    // Allow 20% tolerance
    assert!(bit_diff > 100, "Only {} bits differ, expected ~128", bit_diff);
}

#[test]
fn test_different_trees_different_roots() {
    let data_sizes = vec![2, 4, 8, 16];
    
    for size in data_sizes {
        let data1: Vec<&[u8]> = (0..size)
            .map(|i| format!("tree1_item_{}", i))
            .map(|s| Box::leak(s.into_boxed_str()).as_bytes())
            .collect();
        
        let data2: Vec<&[u8]> = (0..size)
            .map(|i| format!("tree2_item_{}", i))
            .map(|s| Box::leak(s.into_boxed_str()).as_bytes())
            .collect();
        
        let tree1 = MerkleTree::new(data1);
        let tree2 = MerkleTree::new(data2);
        
        assert_ne!(tree1.root(), tree2.root());
    }
}

    //Tier 5: Proof Paths
    #[test]
fn test_proof_path_exists() {
    let tree = MerkleTree::new(vec![b"a", b"b", b"c", b"d"]);
    
    let proof = tree.proof_path(0);
    assert!(proof.is_some());
    assert_eq!(proof.unwrap().len(), 2); // Depth = 2 => 2 siblings
}

#[test]
fn test_proof_path_length_matches_depth() {
    for size in &[2, 4, 8, 16] {
        let data: Vec<&[u8]> = (0..*size)
            .map(|i| format!("item_{}", i))
            .map(|s| Box::leak(s.into_boxed_str()).as_bytes())
            .collect();
        
        let tree = MerkleTree::new(data);
        
        for leaf_idx in 0..*size {
            if let Some(proof) = tree.proof_path(leaf_idx) {
                assert_eq!(proof.len(), tree.depth());
            }
        }
    }
}

    // Tier 6: End-to-End Verification
    #[test]
    fn test_full_cycle_verification() {
        let data:Vec<&[u8]> = vec![b"a", b"b", b"c", b"d"];
        let tree = MerkleTree::new(data.clone());
        let root = tree.root();
        
        // Generate proof for leaf "c" at index 2
        let proof = tree.proof_path(2).expect("Proof should exist for index 2");
        
        // Success case: Verify the legitimate leaf
        assert!(verify_proof(b"c", &proof, root), "Verification should pass for valid leaf and proof");
        
        // Failure case: Verify that a modified leaf fails even with the correct proof structure
        assert!(!verify_proof(b"wrong", &proof, root), "Verification should fail for incorrect leaf data");
    }

}