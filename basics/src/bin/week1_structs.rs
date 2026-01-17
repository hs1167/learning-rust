use std::fmt;


#[derive(Clone, Copy,Debug)]
pub struct Note{
    pub secret: u64,
    pub nullifier: u64,
}

impl Note{


    pub fn new(secret:u64,nullifier:u64) -> Self {
        Note{secret: secret, nullifier: nullifier}
    }

    pub fn commitment(&self) -> u64 {
        self.secret ^ self.nullifier
    }
}

impl fmt::Display for Note {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Note(secret={}, nullifier={})", self.secret, self.nullifier)
    }
}
fn main(){
    let note = Note{secret:123456, nullifier: 789};
    println!("{:?}",note);
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test] 
    fn test_note_creation(){
        let note = Note::new(42,100);
        assert_eq!(note.commitment(),42^100);
    }

    #[test]
    fn test_commitment_deterministic() {
        let n1 = Note::new(42, 100);
        let n2 = Note::new(42, 100);
        assert_eq!(n1.commitment(), n2.commitment());
    }

    #[test]
    fn test_display() {
        let note = Note::new(42, 100);
        println!("{}", note);  // Should print: Note(secret=42, nullifier=100)
    }
}

