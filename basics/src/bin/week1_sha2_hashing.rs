use sha2::{Sha256, Digest};

fn hash_note (secret: u64, nullifier: u64) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(secret.to_le_bytes());
    hasher.update(nullifier.to_le_bytes());
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

fn main(){

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_determinism() {
        let secret = 42;
        let nullifier = 100;

        let h1 = hash_note(secret, nullifier);
        let h2 = hash_note(secret, nullifier);

        assert_eq!(h1,h2,"hashes has to be deterministic");
    }

    #[test]
    fn test_hash_collision_resistance() {
        let h1 = hash_note(42,100);
        let h2 = hash_note(42,101);
        assert_ne!(h1,h2,"hashes has to bee collision resistent")
    }
}
