use std::any::{Any, TypeId};

fn main() {
    let x: &dyn Any = &42;

    // Rusts equivalent of typeid always forces runtime evaluation
    if x.type_id() == TypeId::of::<i32>() {
        println!("x is an i32");
    } else {
        println!("x is not an i32");
    }
}