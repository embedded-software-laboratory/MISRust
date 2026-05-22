unsafe fn no_lifetime_extension_within_unsafe_possible(x: &i32) -> &'static i32 {
    // Fails to compile.
    // unsafe {
    //     x
    // }

    &5 // comment out to check.
}

unsafe fn bypass_borrowing_rules(x: &i32) -> *const i32 {
    x as *const i32
}


fn main() {
    unsafe {
        let _ = no_lifetime_extension_within_unsafe_possible(&1);
    }
        
    let ptr: *const i32;
    {
        let value = 5;
        unsafe {
            ptr = bypass_borrowing_rules(&value);
        }
    }
    unsafe {
        // miri returns UB due to dangling pointer dereference.
        println!("{}", *ptr);
    }
}