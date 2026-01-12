fn main() {
    let x = plus_one(5); //we cant assign plus_one... to the following statement : let x = ...

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 //there we have an error cause we dont return anything, because of the semicolon, its just an statement
}