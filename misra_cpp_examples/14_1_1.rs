pub struct MyStruct {
    pub public_field: i32,
    private_field: i32,
}

impl MyStruct {
    pub fn new(public_value: i32, private_value: i32) -> MyStruct {
        MyStruct {
            public_field: public_value,
            private_field: private_value,
        }
    }

    pub fn get_private_field(&self) -> i32 {
        self.private_field
    }
}

fn main() {
    let my_struct = MyStruct::new(10, 20);

    println!("Public field: {}", my_struct.public_field);
    println!("Private field: {}", my_struct.get_private_field());
}