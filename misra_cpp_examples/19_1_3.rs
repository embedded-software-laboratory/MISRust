// Non-existing feature: compilation WARNING: unexpected 'cfg' condition value
// compiler note: expected values for 'feature' are: ['list', 'of', 'existing', 'features']
#[cfg(feature = "feature_does_not_exist")]
fn feature_function() {
    println!("Feature is enabled!");
}

fn main() {
    // compilation error: cannot find function 'feature_function' in this scope
    feature_function();
}
