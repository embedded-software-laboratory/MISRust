fn main() {
    let mut x = 0;

    if true && { x += 1; x > 0 } {
        println!("x is now {}", x);
    }
}