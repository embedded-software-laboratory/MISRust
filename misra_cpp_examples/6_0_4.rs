mod non_entrypoint_main {
    // non-compliant use of name main
    pub fn main() {}
}
fn main() {
    use non_entrypoint_main;
    non_entrypoint_main::main();
}
