fn takes_array(array: [i32; 3]) {
    println!("Array: {:?}", array);
}
fn takes_slice(slice: &[i32]) {
    println!("Slice: {:?}", slice);
}

fn main() {
    let array: [i32; 3] = [1, 2, 3];
    // pass as reference
    takes_slice(&array);
    // pass as value
    takes_array(array);
}
