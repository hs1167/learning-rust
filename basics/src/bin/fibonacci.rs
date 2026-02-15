fn main(){
    let x = fib(3);
    println!("The 18th fibonacci number is : {x}");
}

fn fib(n:u32) -> u32 {
    if n==0 {0} else if n==1 {1} else{
        fib(n-1) + fib(n-2)
    }
}