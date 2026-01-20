use sha2::{Sha256, Digest};

// Define a type alias for readability: a fixed-size array on the stack
pub type Hash = [u8; 32];

pub struct MerklePath {pub hashes: Vec<Hash>,}

impl MerklePath {
    //Declaration of verify method
    pub fn verify(&self, root: Hash, leaf: Hash, mut index: usize) -> bool {
        let mut current_hash = leaf;
        for (idx, &sibling) in self.hashes.iter().enumerate() {

            if index%2 == 0 {current_hash = hash_pair(current_hash, sibling)} 
                else {current_hash = hash_pair(sibling, current_hash)}
            index /= 2;
        }
        if current_hash == root {true} else {false}
    }
}

fn hash_pair(left: Hash, right: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(&left);
    hasher.update(&right);
    let res = hasher.finalize();
    res.try_into().expect("digest size mismatch")
}
fn main(){
    //
}

#[cfg(test)]
mod tests_crypto {
    use super::*;
    use sha2::{Sha256, Digest};

    /// Helper function for tests: manually hash raw bytes using SHA-256
    fn sha256_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize()
            .try_into()
            .expect("SHA-256 always produces 32 bytes")
    }

    #[test]
    fn test_hash_pair_non_commutative() {
        // Test that hash_pair is non-commutative (order matters)
        let left = [0xAAu8; 32];
        let right = [0xBBu8; 32];

        let hash_lr = hash_pair(left, right);
        let hash_rl = hash_pair(right, left);

        assert_ne!(hash_lr, hash_rl, "Hashing is non-commutative; order must be respected");
    }

    #[test]
    fn test_hash_pair_consistency() {
        // Test that hash_pair is deterministic
        let left = [0x11u8; 32];
        let right = [0x22u8; 32];

        let hash_1 = hash_pair(left, right);
        let hash_2 = hash_pair(left, right);

        assert_eq!(hash_1, hash_2, "Hashing the same pair twice must yield the same result");
    }

    #[test]
    fn test_verify_valid_proof() {
        // Manually construct a small 4-leaf Merkle tree and verify a proof

        // Step 1: Define 4 leaf values and hash them
        let leaf_0 = sha256_hash(b"leaf_0");
        let leaf_1 = sha256_hash(b"leaf_1");
        let leaf_2 = sha256_hash(b"leaf_2");
        let leaf_3 = sha256_hash(b"leaf_3");

        // Step 2: Compute level-1 parents
        let parent_0_1 = hash_pair(leaf_0, leaf_1);  // left node
        let parent_2_3 = hash_pair(leaf_2, leaf_3);  // right node

        // Step 3: Compute root (level 2)
        let root = hash_pair(parent_0_1, parent_2_3);

        // Step 4: Construct proof for leaf_0 (index 0)
        // At depth 0: sibling is leaf_1
        // At depth 1: sibling is parent_2_3
        let proof_0 = MerklePath {
            hashes: vec![leaf_1, parent_2_3],
        };

        // Verify the proof for leaf_0
        assert!(
            proof_0.verify(root, leaf_0, 0),
            "Proof for leaf_0 should be valid"
        );

        // Verify that an invalid root is rejected
        let invalid_root = [0xFFu8; 32];
        assert!(
            !proof_0.verify(invalid_root, leaf_0, 0),
            "Proof should fail with incorrect root"
        );
    }

    #[test]
    fn test_verify_leaf_1_proof() {
        // Verify leaf_1 in the same tree (4 leaves)

        let leaf_0 = sha256_hash(b"leaf_0");
        let leaf_1 = sha256_hash(b"leaf_1");
        let leaf_2 = sha256_hash(b"leaf_2");
        let leaf_3 = sha256_hash(b"leaf_3");

        let parent_0_1 = hash_pair(leaf_0, leaf_1);
        let parent_2_3 = hash_pair(leaf_2, leaf_3);
        let root = hash_pair(parent_0_1, parent_2_3);

        // Proof for leaf_1 (index 1): at depth 0, sibling is leaf_0 (and it's to the LEFT)
        let proof_1 = MerklePath {
            hashes: vec![leaf_0, parent_2_3],
        };

        assert!(
            proof_1.verify(root, leaf_1, 1),
            "Proof for leaf_1 should be valid with leaf_0 on the left"
        );
    }

    #[test]
    fn test_verify_leaf_2_proof() {
        // Verify leaf_2 (index 2) in the same 4-leaf tree

        let leaf_0 = sha256_hash(b"leaf_0");
        let leaf_1 = sha256_hash(b"leaf_1");
        let leaf_2 = sha256_hash(b"leaf_2");
        let leaf_3 = sha256_hash(b"leaf_3");

        let parent_0_1 = hash_pair(leaf_0, leaf_1);
        let parent_2_3 = hash_pair(leaf_2, leaf_3);
        let root = hash_pair(parent_0_1, parent_2_3);

        // Proof for leaf_2 (index 2): sibling is leaf_3, then parent_0_1
        let proof_2 = MerklePath {
            hashes: vec![leaf_3, parent_0_1],
        };

        assert!(
            proof_2.verify(root, leaf_2, 2),
            "Proof for leaf_2 should be valid"
        );
    }

    #[test]
    fn test_verify_wrong_sibling() {
        // Test that the proof fails if a sibling hash is corrupted

        let leaf_0 = sha256_hash(b"leaf_0");
        let leaf_1 = sha256_hash(b"leaf_1");
        let leaf_2 = sha256_hash(b"leaf_2");
        let leaf_3 = sha256_hash(b"leaf_3");

        let parent_0_1 = hash_pair(leaf_0, leaf_1);
        let parent_2_3 = hash_pair(leaf_2, leaf_3);
        let root = hash_pair(parent_0_1, parent_2_3);

        // Corrupted proof: wrong sibling at depth 0
        let corrupted_proof = MerklePath {
            hashes: vec![[0xCCu8; 32], parent_2_3],
        };

        assert!(
            !corrupted_proof.verify(root, leaf_0, 0),
            "Corrupted proof should fail verification"
        );
    }
}