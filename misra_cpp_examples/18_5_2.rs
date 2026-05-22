use std::process;

fn main() {
    println!("This will terminate the program.");
    process::exit(1); // Exit with status code 1
    println!("Unreachable location");
}