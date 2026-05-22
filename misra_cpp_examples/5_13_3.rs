fn main() {
    let not_an_octal = 0345;
    println!("Prints 345: {}", not_an_octal);

    let an_octal = 0o345;
    println!("\nPrints 229: {}", an_octal);
}
