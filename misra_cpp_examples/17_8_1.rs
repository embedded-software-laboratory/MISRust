// Using this generic function would result in a compliation error
// error[E0428]: the name 'generic' is defined multiple times
// fn generic<T>(arg: T) -> () {}

fn generic<i64>(arg: i64) -> () {}
fn main() {
    let s = generic::<i64>(6);
}
