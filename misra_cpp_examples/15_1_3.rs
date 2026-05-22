struct SomeStruct(i64);

impl SomeStruct {
    // Associated function to create a new SomeStruct
    pub fn new(value: i64) -> SomeStruct {
        SomeStruct(value)
    }

    // Method to get the value
    pub fn value(&self) -> i64 {
        self.0
    }
}

fn main() {
    let my_struct = SomeStruct::new(42);
    println!("The value is: {}", my_struct.value());
}