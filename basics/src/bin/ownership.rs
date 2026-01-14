
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0; //here the ownership of t on the hello string is partially moved

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1); //here t.1 already exists, but not the entire t tuple!
}