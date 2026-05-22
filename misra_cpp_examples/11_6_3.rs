enum MyEnum {
    Variant1 = 1, // Implicitly assigned value 0
    Variant2,     // Implicitly assigned value 2
    // Compilation error: Discriminant value '2' is assigned more than once
    Variant3 = 2, // Explicitly assigned value 2
}

fn main() {
    // Use the enum variants
    let v1 = MyEnum::Variant1 as i32;
    let v2 = MyEnum::Variant2 as i32;
    let v3 = MyEnum::Variant3 as i32;

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 1);
}
