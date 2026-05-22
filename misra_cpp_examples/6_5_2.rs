mod some_namespace {
    // non-compliant: external linkage within a namespace
    unsafe extern "C" {
        pub fn external_function(x: i32) -> i32;
    }
}

fn main() {
    // Would compile, if external linkage existed
    unsafe {
        let result = some_namespace::external_function(42);
        println!("Result: {}", result);
    }
}
