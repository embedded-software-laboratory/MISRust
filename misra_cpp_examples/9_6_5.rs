trait Displayable {
    fn display(&self);
}

struct TypeA;

impl Displayable for TypeA {
    fn display(&self) {
        println!("TypeA");
    }
}

// Abstract return type of a Trait with missing return value on one path
fn example_function(condition: bool) -> impl Displayable {
    if condition {
        return TypeA;
    }
    // Compilation error: mismatched types, expected struct `TypeA`, found ()`
    
}

fn main() {
    example_function(true).display();
}
