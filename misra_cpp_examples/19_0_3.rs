fn defined_too_early() -> i32 {
    42
}

use std::marker::PhantomData;

fn main() {
    let _ = PhantomData::<i32>;
    let _ = defined_too_early();
}
