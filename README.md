# 🦀 Learning Rust & Zero-Knowledge Proofs

**Journey Started:** January 10, 2026  
**Status:** Mastering Rust fundamentals & Cryptographic structures.

---

## 🚀 Featured Project: Merkle Tree from Scratch
**Shipped: January 24, 2026** After a week of intensive learning via the *Rust Book* and *Rustlings*, I dedicated the half of second week to designing and implementing a complete **Binary Merkle Tree**.

### Key Technical Achievements:
* **Learning Curve**: Progressed from basic syntax to complex data structures and cryptographic implementations in just 14 days.
* **Bottom-Up Construction**: Efficiently builds tree layers from leaves to root in $O(n)$ time.
* **Cryptographic Integrity**: Leveraged SHA-256 to implement the **Avalanche Effect** (validated via 13+ unit tests).
* **Logarithmic Proofs ($O(\log n)$)**: Implemented `proof_path` for inclusion proofs and a stateless `verify_proof` function.

### Engineering Insights:
* **Ownership & Memory**: Applied Rust's borrow checker principles using slices (`&[u8]`) to ensure memory safety without performance overhead.
* **Algorithm Design**: Developed parity-based navigation for sibling traversal and proof generation.
* **Stateless Verification**: Decoupled verification logic from the tree storage, mimicking real-world blockchain client behavior.

```mermaid
graph BT
    L1[Leaf 0] --> P1[Parent 0-1]
    L2[Leaf 1] --> P1
    L3[Leaf 2] --> P2[Parent 2-3]
    L4[Leaf 3] --> P2
    P1 --> R[Merkle Root]
    P2 --> R
    style R fill:#f96,stroke:#333,stroke-width:4px

## 📂 Project Structure
* `basics/`: Core Rust concepts and custom data structures (The Merkle Tree lives here).
* `exercises/rustlings/`: Focused on core chapters to build a solid foundation for crypto-implementations.
* `proof-systems/`: Ongoing research into algebraic structures and ZK-SNARKs and ZK-STARKs.
