type A = i32;
static some_var: i32 = 42;
unsafe extern "C" {
    // Compilation error: 'some_var' is defined multiple times
    static some_var: A;
}

fn main() {}
