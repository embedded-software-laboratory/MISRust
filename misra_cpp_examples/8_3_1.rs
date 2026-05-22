fn main() {
    let x: u32 = 42;
    // Compilation error: cannot apply unary operator `-` to `u32`
    let y = -(x as u32);

    println!("x: {}, y: {}", x, y);
}