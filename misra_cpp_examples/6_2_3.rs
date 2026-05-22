type A = i32;
// Compilation error: defined multiple times
type A = i32;

static global_i: i32 = 4;

// Compilation error: defined multiple times
static global_i: i32 = 4;

const GLOBAL_K: i32 = 5;
// Compilation error: defined multiple times
const GLOBAL_K: i32 = 5;

pub trait MyTrait {}
// Compilation error: defined multiple times
pub trait MyTrait {}
fn main() {}
