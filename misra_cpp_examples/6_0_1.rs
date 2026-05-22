type A = i32;

fn main() {
    {
        fn test() -> A {
            42
        }
        let _some_var: A = 42;
        test();
    }
}
