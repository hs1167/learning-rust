fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // 1. ALLOCATION
    // We create a mutable String. 's' is the OWNER of the data in the Heap.
    // Math: s is our set E.
    let mut s = String::from("hello world");

    // 2. IMMUTABLE BORROW (The "Freeze")
    // We create a slice 'word'. It is a reference (&str) pointing to 's'.
    // RULE: While 'word' is alive, 's' is strictly READ-ONLY.
    // Math: 'word' is a projection π(s). It depends on E's structure.
    let word = first_word(&s);
    s.clear(); 
    
    // 3. USAGE (Critical Zone)
    // We use 'word' here. The compiler checks: "Is 's' still valid?" -> YES.
    println!("the first word is: {}", word);

    // --- IMPLICIT SCOPE END FOR 'word' ---
    // Rust's "Non-Lexical Lifetimes" (NLL) logic sees that 'word' 
    // is never used after the line above.
    // The immutable borrow on 's' is released HERE.
    // -------------------------------------

    // 4. MUTABLE BORROW (The "Mutation")
    // Now that the read-lock is released, we can modify 's'.
    // Math: We apply the destructive morphism Φ(E) -> ∅.
    // This works ONLY because 'word' (the projection) is dead.
    
    // If we tried to use 'word' down here, the compiler would scream 
    // because we would be referencing destroyed data (Use-After-Free).
}

