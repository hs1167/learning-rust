fn gcd (mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let mut remainder = a%b;
        a = b;
        b = remainder;
    }
    a
}



fn main () {
    let c = gcd(66528,52920);
    println!("{c}");
}