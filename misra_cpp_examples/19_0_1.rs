fn main() {
    platform_specific_function();
}

// Correct cfg attribute
#[cfg(target_os = "windows")]
fn platform_specific_function() {
    println!("Running on Windows");
}

// Typo in cfg attribute (target_os is misspelled as target_osx)
#[cfg(target_osx = "linux")]
fn platform_specific_function() {
    println!("Running on Linux");
}

// Fallback function for unsupported platforms
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn platform_specific_function() {
    println!("Running on an unsupported platform");
}