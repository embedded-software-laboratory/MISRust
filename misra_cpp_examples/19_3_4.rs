macro_rules! add_multiply {
    // Metavariable expression
    ($a:expr, $b:expr) => {
        $a * $b
    };
}

fn ident(num: i32) -> i32 {
    num
}
fn main() {
    let x = 2;
    let y = 3;

    // Without parentheses, the macro would produce unexpected results in C, but in Rust
    // the parameters are matched as one indivisible expression token, and evaluated correctly
    // C++: this macro would expand to 1 + 2 * 3 = 7
    // Rust: 1+2 is expression $a, 3 is expression $b, both are evaluated seperately
    let result1 = add_multiply!(ident(1) + 2, 3);
    println!("Result without parentheses: {}", result1); // Output: 9

    // With parentheses, the calculation would be:
    // C++:  3 * 3 = 9
    // Rust: 3 * 3 = 9
    let result2 = add_multiply!(ident(3), (3));
    println!("Result with parentheses: {}", result2); // Output: 9
}
