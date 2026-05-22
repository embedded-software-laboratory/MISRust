fn main() {
    let b: i8 = 1;
    let c = b << 256;
    // does not compile
    println!("c = {}", c);
}
