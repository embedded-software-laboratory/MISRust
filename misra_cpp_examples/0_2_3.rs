#![deny(unused)]

mod name_space {
    // (named) module (equivalent to namespace in C++)
    // unused type alias warning
    pub type DeclaredWithPublicVisibility = u64;
}

fn main() {
    {
        // Block scope
        // unused type alias warning
        type DeclaredWithLimitedVisibility = u64;
    }
    println!("Type was not used");
}
