fn main() {
    let a = 10;
    let b = 20;

    let a_ptr: *const i32 = &a;
    let b_ptr: *const i32 = &b;

    if a_ptr < b_ptr {
        println!("a_ptr is less than b_ptr");
    } else {
        println!("a_ptr is not less than b_ptr");
    }
}