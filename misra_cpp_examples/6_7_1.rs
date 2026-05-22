fn main() {
    static mut test_var: i32 = 0;
    // Compilation error: use of mutable static requires unsafe function or block
    test_var += 1;
}
