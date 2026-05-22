mod fun_with_int_argument {
    // non-compliant use of name main
    pub fn function(a: i32) -> () {}
}

mod fun_with_char_argument {
    pub fn function(a: char) -> () {}
}
fn main() {
    use fun_with_char_argument::*;
    use fun_with_int_argument::*;
    // Compilation error: 'function' is ambiguous
    function(3);
}
