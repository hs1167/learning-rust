use sha2::{Sha256, Digest};

/// Computes a SHA-256 hash of a secret and a nullifier to create a unique note commitment.
/// In the context of a privacy mixer, this commitment is stored on-chain while the
/// preimage remains known only to the depositor.
fn hash_note (secret: u64, nullifier: u64) -> [u8;32] {
    // Initialize the SHA-256 hasher instance.
    // The 'mut' keyword is required as hashing is an incremental state-update process.
    let mut hasher = Sha256::new();
    
    // Feed data into the hasher. Use Little-Endian (LE) byte ordering for 
    // consistent behavior across different CPU architectures and ZK VMs.
    hasher.update(secret.to_le_bytes());
    hasher.update(nullifier.to_le_bytes());
    
    // Perform final padding and finalize the hash computation.
    let result = hasher.finalize();
    
    // Convert the generic Output array into a standard Rust [u8; 32] array.
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

fn main(){
    // Entry point for the binary. Logic for deposits/withdrawals will be implemented here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Verifies that the hashing process is deterministic.
    /// Determinism is a strict requirement for ZK proof systems like SP1 to ensure 
    /// the prover and verifier can arrive at the same commitment.
    fn test_hash_determinism() {
        let secret = 42;
        let nullifier = 100;

        let h1 = hash_note(secret, nullifier);
        let h2 = hash_note(secret, nullifier);

        assert_eq!(h1,h2,"hashes has to be deterministic");
    }

    #[test]
    /// Verifies that different inputs result in different hash outputs.
    /// This ensures collision resistance, preventing users from spending the 
    /// same note or predicting other users' commitments.
    fn test_hash_collision_resistance() {
        let h1 = hash_note(42,100);
        let h2 = hash_note(42,101);
        assert_ne!(h1,h2,"hashes has to bee collision resistent")
    }
}