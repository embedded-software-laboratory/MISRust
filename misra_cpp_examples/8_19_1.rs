fn times_two(value : i32) -> i32 {
    value * 2
}

fn main() {
    let mut x = 5;
    // Sequenced expression is clear and concise
    let y = times_two({x += 1; x});
    // C++ Syntax:
    // y = times_two((x += 1, x));
    assert_eq!(y, 12);
}