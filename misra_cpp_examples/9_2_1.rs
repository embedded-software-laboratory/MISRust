use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(Mutex::new());
    {
        // Lock the mutex until the end of this block
        data.lock().unwrap(); 
        // error-prone C++ conversion syntax:
        // std::scoped_lock { a_mutex };
        // less ambigous Rust conversion syntax:
        // x as U
    }
    println!("Mutex is unlocked now");
}