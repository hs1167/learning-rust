fn main() {
    let y = {
        let x = 3;
        x + 1  //if we put a semicolon here, we turn expression into a statement, and it will then not return a value
    };

    println!("The value of y is: {y}");
}

//reminder 
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.
