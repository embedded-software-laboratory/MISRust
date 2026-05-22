trait Base {
    fn base_fn(&self) -> () {
        println!("Base function");
    }
}
struct DerivedBase;
impl Base for DerivedBase {}

impl DerivedBase {
    fn base_fn(&self) -> () {
        println!("DerivedBase function");
    }
}

fn main() {
    let derived_base = DerivedBase;
    // Prints: DerivedBase function
    derived_base.base_fn();
    // Prints: Base function
    <DerivedBase as Base>::base_fn(&derived_base);
}
