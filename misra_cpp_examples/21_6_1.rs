fn main() {
    unsafe {
        // Allocate memory for an integer on the heap using Box
        let boxed_value = Box::new(42);

        // Convert the Box into a raw pointer
        let ptr: *mut i32 = Box::into_raw(boxed_value);

        // Intentionally forget to deallocate the memory, causing a memory leak
        // Box::from_raw(ptr); // This line is commented out to create a memory leak

        // Access the allocated memory
        println!("Value: {}", *ptr);

        // Normally, you should deallocate the memory to avoid a memory leak
        // Box::from_raw(ptr); // This would convert the raw pointer back into a Box and deallocate it
    }
}
