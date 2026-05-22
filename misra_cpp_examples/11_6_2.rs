use std::mem::MaybeUninit;

fn main() {
    let x: MaybeUninit<u32> = MaybeUninit::uninit();

    let y = unsafe { x.assume_init() };
    println!("Value: {}", y);
}
