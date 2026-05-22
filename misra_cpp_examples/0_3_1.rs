fn main() {
    let f1: f32 = 1e38_f32;
    let f2: f32 = 5.2343249027384902374902374902378492034890;
    let f3: f32 = 0.0000123419989999999;

    // No warning given for use of suspicious float arithmetic
    let f4: f32 = (f1 * f2) * f3;
    let f5: f32 = f1 * (f2 * f3);
    // gives inf
    println!("f4: {}\n", f4);
    // gives 6460203000000000000000000000000000
    println!("f5: {}\n", f5);

    let oh_no: f64 = 10.0;
    let zero: f64 = 0.0;
    let help = oh_no / zero;
    // gives inf
    println!("Division by zero: {}\n", help);
}
