fn main() {
    let y = 5;
    let z = 6;
    let runtime_assert_result = assert!(y != z);

    const x: i32 = 5;
    // This will not compile, as the assertion evaluates to false during compile time
    const COMPILE_TIME_ASSERT_RESULT: () = assert!(x == 4);
}
