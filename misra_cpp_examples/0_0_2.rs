fn main() {
    let a = 1;
    if a == 2 {
        print!("Hello, this should never reach");
    }

    while a == 1 {
        print!("Hello infinite loop");
    }
}
