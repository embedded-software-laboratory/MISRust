use core::arch::asm;
fn main() {
    unsafe {
        asm!("nop");
    }
}