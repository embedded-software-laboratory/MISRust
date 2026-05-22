fn main() {
    // compile time initialization is checked:
    // Compilation error: literal out of range for 'u8'
    //let not_possible: u8 = 300;

    let mut counter: u8 = 0;
    for i in 0..256 {
        // Will panic during runtime in debug mode only: 
        // attempt to add with overflow
        counter += 1;
    }
    println!("Counter: {}", counter);
}
