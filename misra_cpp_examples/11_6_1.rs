use std::mem::MaybeUninit;

fn main() {
    // Create an uninitialized variable
    let mut uninit: MaybeUninit<i32> = MaybeUninit::uninit();

    unsafe {
        // Initialize the variable
        uninit.as_mut_ptr().write(42);

        // Read the value
        let value = uninit.assume_init();
        println!("Initialized value: {}", value);
    }
}
