fn main() {
    let mut x: &i32 = &42;
    {
        let local = 42;
        // Compilation error: 'local' does not live long enough
        x = &local;
    }
    print!("{}", x);
}
