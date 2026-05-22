extern crate lib1;
extern crate lib2;

fn main() {
    let value = lib1::lib();
    println!("Value from lib1: {}", value);

    let value = lib2::lib();
    println!("Value from lib2: {}", value);
}
