#![deny(unused)]

fn return_res(change_me: &mut u64) -> Result<i32, ()> {
    *change_me = 5;
    Err(())
}

fn return_something(change_me: &mut u64) -> u64 {
    *change_me = 5;
    7
}

fn main() {
    let mut change_me = 0;
    // Throws warning for unused Result
    return_res(&mut change_me);
    // No warning for unused u64
    return_something(&mut change_me);
}
