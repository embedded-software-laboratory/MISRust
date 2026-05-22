use std::ops::Add;

#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: i32,
}

// Implementing the Add trait for MyStruct
impl Add for MyStruct {
    type Output = MyStruct;
    // Strongly typed
    fn add(self, other: MyStruct) -> MyStruct {
        MyStruct {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a = MyStruct { x: 10, y: 20 };
    let b = MyStruct { x: 30, y: 40 };
    let c = a + b; // Using the overloaded + operator
    assert_eq!(c.x, 40);
    assert_eq!(c.y, 60);
}