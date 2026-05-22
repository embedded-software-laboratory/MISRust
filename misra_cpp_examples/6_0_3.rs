// function without namespace
pub fn non_compliant_function() {
    println!("Hello from non_compliant_function");
}

mod module_a {
    pub fn function() {}
}

mod module_b {
    pub fn function() {}
}

fn main() {
    use module_a::*;
    use module_b::*;
    // compilation error: 'function' is ambiguous
    function();
}
