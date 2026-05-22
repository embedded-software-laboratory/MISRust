mod name_space {
    // (named) module (equivalent to namespace in C++)
    // unused function produces compiler warning
    pub fn unused_fn() {}
}

pub struct MyStruct {
    // public struct
    pub field: i32,
}

impl MyStruct {
    // does produce an unused warning
    fn unused_fn() {}

    // does not produce an unusued warning, because the visibility is public.
    pub fn unused_member_fn(self) {}
}

fn main() {
    let _ = MyStruct { field: 0 };
}
