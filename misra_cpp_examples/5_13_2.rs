fn main() {
    // Compilation error: unknown character escape
    println!("Unicode escape requires braces: \u{001F6}00, clearly showing the termination of the escape sequence");
    // Hexadecimal Escape Sequence does not have a clear termination -> Non-compliant
    println!("\x41BCD");
}
