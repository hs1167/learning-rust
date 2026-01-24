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
            let mut part = layers.last().unwrap().chunks_exact(2);
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
}