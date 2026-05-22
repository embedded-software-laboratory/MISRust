fn main() {
    let mut x = 5;
    let y = 10;
    // z is the result of an assignment
    let z = (x = y);

    assert_eq!(z, ());
}