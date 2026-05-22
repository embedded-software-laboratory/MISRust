fn main() {
    let max_u8: u8 = 255;
    let wrapped_result = max_u8.wrapping_add(1);
    // Output: 0
    println!("Wrapped result: {}", wrapped_result);

    // The following line will cause a compilation error
    // Compilation error: this arithmetic operation will overflow
    // let error_no_wrap: u8 = max_u8 + 1;
}
