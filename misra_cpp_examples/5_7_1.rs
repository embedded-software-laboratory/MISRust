/* comment ommitting comment end marker, critical_fn gets included in comment by mistake
pub fn critical_fn() {}

/* another comment, unrelated to the first one */ 

fn main() {
    print!("Hello World");
}

// Throws unterminated block comment error error[E0758], as every opened comment block is expected to have a closing marker
