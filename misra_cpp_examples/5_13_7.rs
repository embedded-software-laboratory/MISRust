fn main() {
    // This will not compile because different types of string literals cannot be concatenated
    //let concatenated = "Hello, " + r"world!"; // Error: mismatched types

    // Correct usage: concatenate same type of string literals
    let concatenated = "Hello, ".to_string() + "world!";
    println!("{}", concatenated);

    // Correct usage: concatenate raw string literals
    let raw_concatenated = r"Hello, ".to_owned() + r"world!";
    println!("{}", raw_concatenated);

    // Correct usage: concatenate byte string literals
    let byte_concatenated = b"Hello, ".to_vec().extend_from_slice(b"world!");
    println!("{:?}", byte_concatenated);
}
