fn main() {
    // gives unused variable warning
    let shadowed_j = 1;
    {
        let shadowed_j = 2;
        //prints 2
        println!("inner_j: {}", shadowed_j);
    }
}
