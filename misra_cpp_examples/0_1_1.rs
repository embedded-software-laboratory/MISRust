#![deny(unused_assignments, unused_variables)]

fn get_num() -> i64 {
    return 5;
}

fn main() {
    let mut double_write_before_read = get_num();
    // write again before reading, throws warning during compilation!
    double_write_before_read = 10;
    println!(
        "I was unneccessarily written to: {}",
        double_write_before_read
    );

    // No warning, although an overwrite occus before reading/observing the incremented value in line 18
    let mut increment_without_effect = 0;
    for _ in 0..10 {
        increment_without_effect = 10;
        increment_without_effect += 1;
    }
    println!(
        "I was incremented without effect: {}",
        increment_without_effect,
    );

    {
        // causes unused variable compilation warning
        let i_just_get_destroyed = get_num();
        let me_too: [u8; 10] = [0; 10];
    }

    // No warning
    let i_get_observed_and_read = get_num();
    let observer = i_get_observed_and_read;
    println!("I was read and observed: {}", observer);
}
