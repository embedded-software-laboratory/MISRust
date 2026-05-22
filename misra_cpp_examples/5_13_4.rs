fn main() {
    // Unsigned integer literals with appropriate suffixes
    let small_number: u8 = 42u8;
    let medium_number: u32 = 1000u32;
    let large_number: u64 = 1_000_000u64;

    println!("Small number: {}", small_number);
    println!("Medium number: {}", medium_number);
    println!("Large number: {}", large_number);

    // defaults to i32 without type hint in a 64bit environment
    let without_suffix = 55;
}
