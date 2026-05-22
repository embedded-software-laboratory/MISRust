fn main() {
    let x = 10;
    let mut y = 20;

    // Call the function with both immutable and mutable references
    process_values(&x, &mut y);

    println!("y after processing: {}", y);
}

// Explicit mut keyword is required for mutable references and variables
fn process_values(immutable_ref: &i32, mutable_ref: &mut i32) {
    *mutable_ref += *immutable_ref;
}