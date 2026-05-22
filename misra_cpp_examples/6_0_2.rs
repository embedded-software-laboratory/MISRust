// not possible to have an external linkage of an array without explicit size
// unsafe extern "C" {
//     // compiler error: missing type for static
// static EXTERNAL_ARRAY;
// }

unsafe extern "C" {
    // no compiler error with explicit size
    static _EXTERNAL_ARRAY: [u8; 10];
}

fn main() {}
