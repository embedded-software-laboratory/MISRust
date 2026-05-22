// compilation error 1: Expected lifetime for return parameter, e.g. &'a u64
// fn invalid_return() -> &u64 {
//     let x = 42;
//     // compilation error 2: Cannot return reference to local variable
//     &x
// }

fn invalid_return_ptr() -> *const u64 {
    let x = 42u64;
    // warning: a dangling pointer will be produced because the local variable `x` will be dropped
    &x as *const u64
}

fn main() {}
