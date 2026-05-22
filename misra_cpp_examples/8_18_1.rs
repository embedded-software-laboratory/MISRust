fn main() {
    let mut array = [1, 2, 3, 4, 5];
    // Compilation error: cannot borrow `array` as mutable because it is also borrowed as immutable
    let src = &array[0..3];
    let dst = &mut array[2..5];

    // std::ptr:copy requires unsafe block
    unsafe {
        std::ptr::copy(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    // with copy_from_slice no overlap possible, due to borrow rules
    dst.copy_from_slice(&src);

    println!("{:?}", array);
}