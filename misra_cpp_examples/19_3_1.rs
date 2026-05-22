macro_rules! example_macro {
    ($a:ident, $b:ident) => {{
        // In Rust version 1.82.0, the only macro as powerful as the # and ## operator in C++ is concat_idents!() which is unstable.
        // The concat_idents!() macro is used to concatenate two identifiers into a single identifier.

        // One possiblity to combine the identifiers is given here as a macro, but in reality offers only disadvantages over a function call
        let a_str = $a.to_string();
        let b_str = $b.to_string();
        let combined_str = format!("{}{}", a_str, b_str);
        combined_str.parse::<i32>().unwrap()
    }};
}

fn main() {
    let x = 10;
    let y = 1;
    let result = example_macro!(x, y);
    assert_eq!(result, 101);
}
