fn main() {
    let b: u8 = 1;
    let c: i8 = -2;
    // error: cannot add 'i8' to 'u8'
    let d = b + c;
    // error: mismatched types, expected 'i8' for variable b, but found 'u8'
    let e = c > b;

    // Explicit cast allowed
    let f = b + c as u8;
    // Prints: f = 255
    print!("f = {}", f);
}
