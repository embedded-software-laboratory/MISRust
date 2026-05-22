#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: i32,
}

// Clone trait implementation with explicit signature
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        MyStruct {
            x: self.x,
            y: self.y,
        }
    }
}

// Implementing Copy, requires Clone trait
impl Copy for MyStruct {}

fn main() {
    let a = MyStruct { x: 10, y: 20 };
    let b = a; // a is copied to b using the Copy trait

    // Both a and b can be used independently, without implementation
    // of the Copy trait, a would be moved to b, resulting in a compilation error
    println!("a: {:?}", a); // Outputs: a: MyStruct { x: 10, y: 20 }
    println!("b: {:?}", b); // Outputs: b: MyStruct { x: 10, y: 20 }
}