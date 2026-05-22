#![deny(unreachable_code, unused_variables)]

fn return_res(change_me: &mut u64) -> Result<(), ()> {
    *change_me = 5;
    Err(())
}
fn main() {
    let mut change_me = 0;
    let unused_result = return_res(&mut change_me);
    print!("change_me: {}", change_me);
    return;
    print!("This generates an unreachable code warning");
}
