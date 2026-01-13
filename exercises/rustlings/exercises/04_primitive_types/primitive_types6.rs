fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
        let second = numbers.1;  //0,1,2,... indexing like in array or sum in math

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
