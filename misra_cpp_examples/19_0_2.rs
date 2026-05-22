macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let result = add!(1, 2);
    // compilation error: cannot add 'char' to 'char'
    //let _ = add!('a', 'b');
    assert_eq!(result, 3);
}
