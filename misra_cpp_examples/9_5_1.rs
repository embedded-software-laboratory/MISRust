fn main() {
    for i in 0..10 {
        // Compilation error: cannot assign twice to immutable variable `i`
        // i =5;
    }

    // Infinite loop
    for i in 0.. {
        println!("{}", i);
    }

}