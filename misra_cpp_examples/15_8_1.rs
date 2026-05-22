#[derive(Debug)]
struct MyStruct {
    x: Box<i32>,
}

// Implementing Clone manually
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        
        // Valid handling of Box type (heap allocated memory) is to clone the value inside the Box
        MyStruct { x: self.x.clone() }
    }
}

// Implementing Copy would result in a compilation error:
// the trait 'Copy' cannot be implemented for this type (MyStruct, because it uses Box)
impl Copy for MyStruct {}

fn main() {
    let a = MyStruct { x: Box::new(10) };
    let b = a.clone(); // a is cloned, because Box can not implement Copy

    // Both a and b can be used independently
    println!("a: {:?}", a); // Outputs: a: MyStruct { x: 10, y: 20 }
    println!("b: {:?}", b); // Outputs: b: MyStruct { x: 10, y: 20 }
}