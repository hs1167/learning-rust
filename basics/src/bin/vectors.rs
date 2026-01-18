fn main(){
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // here we just can remove the borrow &, to use the copy trait on i32

    v.push(6);

    println!("The first element is: {first}");
}