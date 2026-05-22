fn main() {
    // Does not compile, Rust does not have L suffix to denote long integers
    let some_long = 5L;

    // Rust uses explicit suffixes to denote integer types
    let signed_long = 10000000000000000000000i64;
    let unsigned_long = 100000000000000000000u64;
}
