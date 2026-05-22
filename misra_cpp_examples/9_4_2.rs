fn main() {
    let number = 7;

    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=6 => println!("Four to Six"),
        n if n % 2 == 0 => println!("Even number"),
        n if n % 2 != 0 => println!("Odd number"),
        _ => println!("Other"),
    }
}