fn main() {
    // Call the function that never returns
    never_returning_function();
}

fn never_returning_function() -> ! {
    // This function will never return to the caller
    loop {
        println!("This function will run forever.");
    }
}