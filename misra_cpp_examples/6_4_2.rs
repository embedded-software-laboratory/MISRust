trait Base {
    fn base_fn(&self) -> () {
        println!("Base function");
    }
}
struct DerivedBase;
impl Base for DerivedBase {}

struct DerivedOverwrite;
impl Base for DerivedOverwrite {
    fn base_fn(&self) -> () {
        println!("Derived function");
    }
}

fn main() {
    let derived_base = DerivedBase;
    let derived_overwrite = DerivedOverwrite;
    // Prints: Base function
    derived_base.base_fn();
    // Prints: Derived function
    derived_overwrite.base_fn();
}
