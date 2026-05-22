unsafe extern "C" {
    fn external_function(x: i32) -> i32;
}
unsafe extern "C" {
    static EXTERNAL_VARIABLE: i32;
}
fn main() {
    // Compilation error: call to unsafe function must be inside unsafe block
    //let result = external_function(42);

    // Would compile, if external linkage existed
    unsafe {
        let result = external_function(42);
        println!("Result: {}", result);
    }
}
