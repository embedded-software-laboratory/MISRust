struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn create_closure<'a>(&'a self) -> impl Fn() + use<'a> {
        // Closure capturing `&self`
        move || {
            println!("Value: {}", self.value);
        }
    }

    fn change_value(&mut self, value: i32) {
        self.value = value;
    }
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    let closure = my_struct.create_closure();
    // Compilation Error: can not drop, because my_struct is still borrowed (by closure because of moved self)
    drop(my_struct);
    closure();
}
