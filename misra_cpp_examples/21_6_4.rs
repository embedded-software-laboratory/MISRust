struct MyStruct {
    data: *mut i32,
}

impl MyStruct {
    fn new(value: i32) -> Self {
        // Allocate memory on the heap
        let data = Box::into_raw(Box::new(value));
        MyStruct { data }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        // Incorrect: Forgetting to deallocate the memory
        // This will cause a memory leak
        println!("Dropping MyStruct, but not deallocating memory");
    }
}

fn main() {
    let my_struct = MyStruct::new(42);
    // When my_struct goes out of scope, the drop method will be called,
    // but the memory allocated on the heap will not be deallocated.
}
